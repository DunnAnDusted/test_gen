//! A comprehensive function-like macro, for concisely defining parameterized tests.
//!
//! When writing automated tests, it's generally done with two key goals:
//!
//! 1. Comprehensive code coverage
//! 2. Debuggability
//!
//! Out of the box, the design of Rusts standard testing framework,
//! can make comfortably achieving both, a challenge...
//!
//! For most effective debuggability, the design of Rusts testing framework
//! encorages writing separate test functions for every case,
//! as these are each tracked separately against the name of the test function.
//! Unfortunately, this results in the need for a significant deal of boilerplate,
//! to achieved comprehensive code coverage.
//!
//! By constrast, comprehensive code coverage can be achieved easily and concisely,
//! via the use of iteration, but sacrifices clarity and debuggability,
//! by using the same named case, to test potentially many values.
//!
//! `test_gen` is designed to address both goals, by enabling
//! the concise definition of batches of named tests,
//! using a parameterized argument format to minimise the boilerplate
//! otherwise required for specifying batches of tests.
//!
//! # Examples
//!
//! Fruits:
//! ``` no_run
#![doc = doctest_example!("fruits")]
//! ```
//!
//! # License
//!
//! `test_gen` is licensed under the BSD 3-Clause license.
//!
//! See [LICENSE] for details.
//!
//! [LICENSE]: https://github.com/DunnAnDusted/test_gen/blob/main/LICENSE
#![warn(missing_docs)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_quote,
    punctuated::Punctuated,
    token::{Brace, FatArrow, Paren, RArrow},
    Attribute, Error, Expr, Ident, Path, Token, Type,
};

/// A shorthand helper macro, for including test files as documentation examples.
macro_rules! doctest_example {
    ($file:literal) => {
        include_str!(concat!("../tests/doctest_example_", $file, ".rs"))
    };
}

/// Generates unique, named test cases, based on parameterized inputs.
///
/// `test_gen` is designed for generating test functions in bulk, at compile time.
/// Consequently, its syntax is designed to imitating the syntax of a function definition.
///
/// # Examples
///
/// As basic part of the basic requirements for producing test cases,
/// the syntax of `test_gen` always features a number of fixed items.
///
/// These include:
///
/// - A helper function, to drive the behaviour of the test cases
/// - A list of test cases to generate, which includes:
///     - The name of the test case
///     - The arguments which should be passed to the helper function for the case
///
/// Example of basic usage:
/// ``` no_run
#[doc = doctest_example!("assert_result_blocked")]
/// ```
///
/// As noted above, groups of test cases
/// may be driven by similar helper functions to others,
/// where behaviour based on its parameters, would be useful.
///
/// As such, `test_gen` supports block-wide, "static" arguments,
/// which will be passed to the helper function for every test case.
///
/// Example using static arguments:
/// ``` no_run
#[doc = doctest_example!("assert_result_static")]
/// ```
///
/// These required items, can be supplemented with additional items (e.g. Attributes, arbitrary
/// return types) to alter how test cases are evaluated, either block-wide, or on a case-by-case basis.
///
/// Examples of using basic arbitrarty return types:
/// ``` no_run
#[doc = doctest_example!("termination_basic")]
/// ```
///
/// Example of overriding a block-wide arbitrary return type:
/// ``` no_run
#[doc = doctest_example!("termination_override")]
/// ```
/// Note: Arbirary return types follow the same rules of types which can be returned by the `main`
/// function of a binary crate, meaning they must implement the [`Termination`] trait.
///
/// [`Termination`]: std::process::Termination
///
/// Examples of attribute usage:
/// ``` no_run
#[doc = doctest_example!("attributes")]
/// ```
/// Note: The syntax of these examples can be mixed as nessacary, with the exception of `should_panic`
/// in the case of using arbitrary return types, as it isn't supported by Rust's testing framework.
#[proc_macro]
pub fn test_gen(tokens: TokenStream) -> TokenStream {
    // `syn` idioms typically suggest using `parse_macro_input!`
    // for handling the initial token stream parsing,
    // but this obfuscates some of the parsing behaviour.
    //
    // In this case, using `syn::parse`, and funneling the result
    // through a subsequent method chain, makes this implementation clearer here.
    //
    // NOTE: `Into::into` is required here,
    // because `proc_macro2::TokenStream` needs to be converted back
    // to `proc_macro::TokenStream` before it can be returned.
    syn::parse(tokens)
        .map_or_else(Error::into_compile_error, TestHelper::restructure)
        .into()
}

/// The main type, representing the collective structure of `test_gen`.
///
/// This includes:
///
/// * The attributes to apply to every test case
/// * The separator before the helper function
/// * The helper function for driving the tests
/// * The arguments to pass to the helper function for every test
/// * The default return type for the helper function
/// * The fat arrow before the braces surrounding the test cases
/// * The values for producing the resulting test
#[derive(Clone)]
struct TestHelper {
    static_attrs: Vec<Attribute>,
    separator: Separator, // Preserved for span
    helper: Path,
    static_args: Option<FnArgs>,
    static_return_type: Option<ReturnType>,
    farrow: FatArrow, // Preserved for span
    braces: Brace,    // Preserved for span
    cases: Punctuated<TestCase, Token![,]>,
}

impl TestHelper {
    /// Consumes the `TestHelper`, and constructs the token stream of the resulting test cases.
    fn restructure(self) -> TokenStream2 {
        let Self {
            static_attrs,
            separator: _,
            helper,
            mut static_args,
            static_return_type,
            farrow: _,
            braces: _,
            cases,
        } = self;

        // Appends trailing punctuation when static args are specified,
        // to ensure it doesn't conflict with the case specific args,
        // and maps away the parens.
        let static_args = static_args.as_mut().map(|x| {
            x.args.push_punct(Default::default());
            &x.args
        });

        // Mapped to an immutable referance, to ensure the value
        // isn't moved/consumed.
        let static_return_type = static_return_type.as_ref();

        cases
            .into_iter()
            .map(|case| -> TokenStream2 {
                let TestCase {
                    fn_name,
                    colon: _,
                    args:
                        CaseArgs {
                            braces: _,
                            attrs,
                            args,
                            return_type,
                        },
                } = case;

                let args = &args.args;
                // Specifies the default return type as an alternate value.
                let return_type = return_type.as_ref().or(static_return_type);

                // #(#VAR)* syntax behaves similarly to `macro_rules!` equivilent,
                // for items implementing `IntoIterator<Item: ToTokens>`.
                //
                // `return_type` can be referanced directly,
                // due to `ToTokens` being implemented for `Option<T: ToTokens>`,
                // and `ReturnType` including the `->` as part of its `ToTokens` implementation.
                //
                // Separating comma already added to `static_args`,
                // ensuring it's conditional inclusion.
                parse_quote! {
                    #(#static_attrs)*
                    #(#attrs)*
                    #[test]
                    fn #fn_name() #return_type {
                        #helper(#static_args #args)
                    }
                }
            })
            .collect()
    }
}

impl Parse for TestHelper {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parses any attributes to apply to all test cases.
        let static_attrs = input.call(Attribute::parse_outer)?;

        // TODO: A separator preceeding the helper function specification, isn't a great solution,
        // style wise, but does promote a potentially more helpful error message
        // in the case of malformed syntax when specifying attributes...
        let separator = input.parse().map_err(|err| {
            Error::new(err.span(), "expected attributes, `fn`, `struct` or `enum`")
        })?;

        // NOTE: Whilst designed with the intent of specifying a helper *function*,
        // an oddity of the syntax for `test_gen`, means it's also compatible
        // with tuple structs and enum varients! Oop! xD
        let helper = input
            .parse()
            .map_err(|err| Error::new(err.span(), "expected helper function"))?;
        let static_args = input.peek(Paren).then(|| input.parse()).transpose()?;
        let static_return_type = input.call(ReturnType::try_parse)?;
        let farrow = input.parse()?;
        let cases;
        let braces = braced!(cases in input);
        // If the contents of the`cases` is empty,
        // `ParseBuffer::parse_terminated` will simply produce an empty
        // `Punctuated` struct, and no error. On the other hand,
        // `Punctuated::parse_separated_nonempty` falls back on the parsing error
        // for `TestCase`, as well as not allow trailing commas,
        // neither of which is particularly helpful in this case...
        //
        // Instead, its explicitly checked whether `cases` is empty,
        // resulting in a bespoke error which provides an explanation which is actually
        // helpful.
        let cases = cases
            .is_empty()
            .then(|| Error::new(cases.span(), "expected test cases"))
            .map_or_else(|| cases.parse_terminated(TestCase::parse), Result::Err)?;

        let out = TestHelper {
            static_attrs,
            separator,
            helper,
            static_args,
            static_return_type,
            farrow,
            braces,
            cases,
        };

        Ok(out)
    }
}

impl ToTokens for TestHelper {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // `ToTokens` isn't implemented for anything like `&[T: ToTokens]`
        // or `(Into)Iterator<Item: ToTokens>`, so appending them iteratively
        // is about as clean a solution as possible...
        self.static_attrs
            .iter()
            .for_each(|attr| attr.to_tokens(tokens));
        self.separator.to_tokens(tokens);
        self.helper.to_tokens(tokens);
        self.static_args.to_tokens(tokens);
        self.static_return_type.to_tokens(tokens);
        self.farrow.to_tokens(tokens);
        // Token groups are kind of weird, so uses `surround` to identify the tokens which the
        // group should surround...
        self.braces
            .surround(tokens, |inner| self.cases.to_tokens(inner));
    }
}

/// A type representing a separator between the attributes applied to a block of tests,
/// and the helper "function" used (tuple structs and tuple enum variants are also valid, so this
/// type allows for those...)
#[derive(Clone)]
enum Separator {
    Fn(Token![fn]),
    Struct(Token![struct]),
    Enum(Token![enum]),
}

impl Parse for Separator {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![fn]) {
            input.parse().map(Self::Fn)
        } else if lookahead.peek(Token![struct]) {
            input.parse().map(Self::Struct)
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Self::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for Separator {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Fn(item) => item.to_tokens(tokens),
            Self::Struct(item) => item.to_tokens(tokens),
            Self::Enum(item) => item.to_tokens(tokens),
        }
    }
}

/// A type representing an specific test case.
///
/// This includes:
///
/// * The name it will be assigned
/// * The separator colon
/// * The arguments to specify its behaviour (see `CaseArgs` for more detail)
#[derive(Clone)]
struct TestCase {
    fn_name: Ident,
    colon: Token![:], // Preserved for span
    args: CaseArgs,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        // Maps to a bespoke error, as noting that you expect a general "Ident",
        // isn't particularly helpful...
        let fn_name = input
            .parse()
            .map_err(|err| Error::new(err.span(), "expected test case name"))?;
        let colon = input.parse()?;
        let args = input.parse()?;

        let out = TestCase {
            fn_name,
            colon,
            args,
        };

        Ok(out)
    }
}

impl ToTokens for TestCase {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.fn_name.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.args.to_tokens(tokens);
    }
}

/// A type representing values defining the behaviour of a test case.
///
/// This includes:
///
/// * The surrounding braces
/// * The attributes to apply to the specific test case
/// * The arguments to pass to the helper function for the specific test case
/// * The expected return type for the specific test case
#[derive(Clone)]
struct CaseArgs {
    braces: Brace, // Preserved for span
    attrs: Vec<Attribute>,
    args: FnArgs,
    return_type: Option<ReturnType>,
}

impl Parse for CaseArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        let braces = braced!(inner in input);

        let attrs = inner.call(Attribute::parse_outer)?;

        // We can actually usefully validate that we're recieving the next token we expect in this
        // case! Hooray!
        if !inner.peek(Paren) {
            return Err(Error::new(
                inner.span(),
                "expected attributes or function parameters",
            ));
        }

        let args = inner.parse()?;
        let return_type = inner.call(ReturnType::try_parse)?;

        let out = CaseArgs {
            braces,
            attrs,
            args,
            return_type,
        };

        Ok(out)
    }
}

impl ToTokens for CaseArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // All the values for the case are surrounded with braces in this case,
        // so the tokens are all appended within the body of the closure...
        self.braces.surround(tokens, |inner| {
            self.attrs.iter().for_each(|attr| attr.to_tokens(inner));
            self.args.to_tokens(inner);
            self.return_type.to_tokens(inner);
        });
    }
}

/// A type representing a list of arguments, and the parentheise around them.
#[derive(Clone)]
struct FnArgs {
    parens: Paren, // Preserved for span
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for FnArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        let parens = parenthesized!(inner in input);
        // Using `punctuated::parse_separated_nonempty` in this case,
        // because if this type is being parsed, arguments should be expected,
        // and trailing commas look sloppy...
        let args = inner
            .call(Punctuated::parse_separated_nonempty)
            .map_err(|err| Error::new(err.span(), "expected function arguments"))?;

        Ok(Self { parens, args })
    }
}

impl ToTokens for FnArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.parens
            .surround(tokens, |inner| self.args.to_tokens(inner))
    }
}

/// A type representing the right-arrow and function's return type signature.
#[derive(Clone)]
struct ReturnType {
    arrow: RArrow,
    return_type: Type,
}

impl ReturnType {
    /// Conditionally parses the type, if a right-arrow is peeked from the stream.
    ///
    /// Included, due to the optional nature of return types in this macro.
    fn try_parse(input: ParseStream) -> Result<Option<Self>> {
        input.peek(Token![->]).then(|| input.parse()).transpose()
    }
}

impl Parse for ReturnType {
    fn parse(input: ParseStream) -> Result<Self> {
        let arrow = input.parse()?;

        // Default error message is a bit obtuse in this case,
        // so it's mapped to a more specific bespoke error instead.
        let return_type = input
            .parse()
            .map_err(|err| Error::new(err.span(), "expected a return type"))?;

        Ok(Self { arrow, return_type })
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.arrow.to_tokens(tokens);
        self.return_type.to_tokens(tokens);
    }
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadMeDocTestDummy;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_to_tokens<P: Parse + ToTokens>(p: &'static str) {
        let tokens: TokenStream2 = p.parse().expect("string could not be parsed as tokens");

        let parsed = syn::parse_str::<P>(&p)
            .expect("tokens could not be parsed as type")
            .into_token_stream()
            .to_string();

        assert_eq!(parsed, tokens.to_string());
    }

    #[test]
    fn return_type_parsing() {
        parse_to_tokens::<ReturnType>("-> usize");
    }

    #[test]
    fn fn_args_parsing() {
        parse_to_tokens::<FnArgs>("(1, 2)");
    }

    #[test]
    fn case_args_parsing() {
        parse_to_tokens::<CaseArgs>("{ #[ignore] (1, 2) -> usize }");
    }

    #[test]
    fn test_case_parsing() {
        parse_to_tokens::<TestCase>("test: { #[ignore] (1, 2) -> usize }");
    }

    #[test]
    fn test_helper_parsing() {
        parse_to_tokens::<TestHelper>("#[should_panic] fn Into::into -> (usize, usize) => { test: { #[ignore] (1, 2) -> usize } }");
    }
}
