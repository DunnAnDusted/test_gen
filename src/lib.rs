#![no_std]
#![warn(missing_docs)]

/// Generates unique, named test cases, based on parametrised inputs.
///
/// `test_gen` is designed for generating test functions in bulk, at compile time.
/// Consequently, its syntax is designed to imitating the syntax of a function definition.
///
/// # Examples
///
/// The base form of `test_gen` always requires a helper function,
/// followed by one or more test cases, supplied in the form of the case name,
/// and its arguments.
///
/// Example of basic usage:
/// ```
#[doc = include_str!("../tests/doctest_example_square.rs")]
/// ```
///
/// In addition to this, attributes for tests, and arbitrary return types for supporting use
/// with the [`Termination`] trait, are also supported, applied either block-wide, or on a 
/// case-by-case basis.
///
/// Examples of arbitrarty return type usage:
/// ```
#[doc = include_str!("../tests/doctest_example_termination.rs")]
/// ```
///
/// Examples of attribute usage:
/// ```
#[doc = include_str!("../tests/doctest_example_attributes.rs")]
/// ```
/// (Note: The syntax of these examples can be mixed as nessacary, with attributes being applicable
/// to cases with arbitrary return types, with the exception of `should_panic`, as this attribute
/// isn't supported by Rust's testing harness for these cases.)
///
/// [`Termination`]: std::process:Termination
#[macro_export]
macro_rules! test_gen {
    ////////////////////
    // All Case Attrs //
    ////////////////////

    // All Ignore, All Should Panic
    ([ignore, should_panic], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        $crate::test_gen! { [should_panic, ignore], $helper => { $($case_name: { $([$($attr),+], )? ($($case_args),+) }),+ } }
    };

    // All Should Panic, All Ignore
    ([should_panic, ignore], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        $crate::test_gen! { [ignore], $helper => { $($case_name: { [should_panic $(, $($attr),+)?], ($($case_args),+)}),+ } }
    };

    // All Should Panic
    ([should_panic], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { [should_panic $(, $($attr),+)?], ($($case_args),+) }),+ } }
    };

    ////////////////
    // All Return //
    ////////////////

    // All Ignore
    ([ignore], $helper:expr => $return:ty => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { [ignore $(, $($attr),+)?], ($($case_args),+) => $return }),+ } }
    };

    // None
    ($helper:expr => $return:ty => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { $([$($attr),+], )? ($($case_args),+) => $return }),+ } }
    };

    ///////////////////
    // Unique Return //
    ///////////////////

    // All Ignore
    ([ignore], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) $(=> $return:ty)? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { [ignore $(, $($attr),+)?], ($($case_args),+) $(=> $return)? }),+ } }
    };

    // BASE
    ($helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) $(=> $return:ty)? }),+ }) => {
        $(
            $($(#[$attr])+)?
            #[test]
            fn $case_name() $(-> $return)? {
                $helper($($case_args),+)
            }
        )+
    };
}

// Basic tests, to confirm the macro can generate `#![no_std]` compatible code.
#[cfg(test)]
mod no_std_tests {
    test_gen! {
        Into::into => {
            unit: { (()) },
            result: { (Ok(())) => Result<(), ()> }
        }
    }
}
