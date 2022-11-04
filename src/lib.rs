//! A comprehensive declarative macro, for concisely defining parameterized tests.
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
//! By constast, comprehensive code coverage can be achieved easily and concisely,
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
#![doc = include_str!("../tests/doctest_example_fruits.rs")]
//! ```
//!
//! # License
//!
//! `test_gen` is licensed under the [BSD 3-Clause].
//!
//! [BSD 3-Clause]: https://github.com/DunnAnDusted/test_gen/blob/main/LICENSE
#![no_std]
#![warn(missing_docs)]

/// Generates unique, named test cases, based on parameterized inputs.
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
/// ``` no_run
#[doc = include_str!("../tests/doctest_example_square.rs")]
/// ```
///
/// In addition to this, attributes for tests, and arbitrary return types for supporting use
/// with the [`Termination`] trait, are also supported, applied either block-wide, or on a
/// case-by-case basis.
///
/// Examples of arbitrarty return type usage:
/// ``` no_run
#[doc = include_str!("../tests/doctest_example_termination.rs")]
/// ```
///
/// Examples of attribute usage:
/// ``` no_run
#[doc = include_str!("../tests/doctest_example_attributes.rs")]
/// ```
/// (Note: The syntax of these examples can be mixed as nessacary, with attributes being applicable
/// to cases with arbitrary return types, with the exception of `should_panic`, as this attribute
/// isn't supported by Rust's testing harness for these cases.)
///
/// [`Termination`]: https://doc.rust-lang.org/std/process/trait.Termination.html
#[macro_export]
macro_rules! test_gen {
    ////////////////////
    // All Case Attrs //
    ////////////////////

    // All Ignore, All Should Panic
    ([ignore, should_panic], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        test_gen! { [should_panic, ignore], $helper => { $($case_name: { $([$($attr),+], )? ($($case_args),+) }),+ } }
    };

    // All Should Panic, All Ignore
    ([should_panic, ignore], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        test_gen! { [ignore], $helper => { $($case_name: { [should_panic $(, $($attr),+)?], ($($case_args),+)}),+ } }
    };

    // All Should Panic
    ([should_panic], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        test_gen! { $helper => { $($case_name: { [should_panic $(, $($attr),+)?], ($($case_args),+) }),+ } }
    };

    ////////////////
    // All Return //
    ////////////////

    // All Ignore
    ([ignore], $helper:expr => $return:ty => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        test_gen! { $helper => { $($case_name: { [ignore $(, $($attr),+)?], ($($case_args),+) => $return }),+ } }
    };

    // None
    ($helper:expr => $return:ty => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) }),+ }) => {
        test_gen! { $helper => { $($case_name: { $([$($attr),+], )? ($($case_args),+) => $return }),+ } }
    };

    ///////////////////
    // Unique Return //
    ///////////////////

    // All Ignore
    ([ignore], $helper:expr => { $($case_name:ident: { $([$($attr:meta),+], )? ($($case_args:expr),+) $(=> $return:ty)? }),+ }) => {
        test_gen! { $helper => { $($case_name: { [ignore $(, $($attr),+)?], ($($case_args),+) $(=> $return)? }),+ } }
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

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadMeDocTestDummy;

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
