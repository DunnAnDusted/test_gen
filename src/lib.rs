//! A comprehensive declarative macro, for concisely defining parameterized tests.
//!
//! When writing automated tests, we're always aiming for two key criteria:
//!
//! 1. Comprehensive code coverage
//! 2. Ease of identification
//!
//! Out of the box, the design of Rust's test harness can make this something of a pain though...
//!
//! Ease of identification encorages writing separate test functions for every test case, so the
//! harness registers them against separate names. Unfortunately, this results in the need
//! for a significant deal of boilerplate, to achieved comprehensive code coverage.
//!
//! By constast, comprehensive code coverage can be achieved easily and concisely,
//! via the use of iteration, but sacrifices clarity and ease of debugging, by testing many values,
//! with the same named case.
//!
//! `test_gen` is designed to address these goals, by allowing named test cases to be specified in
//! batches with similar behaviour and attributes, but distinct arguments, minimising boilerplate.
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
/// ``` ignore
#[doc = include_str!("../tests/doctest_example_square.rs")]
/// ```
///
/// In addition to this, attributes for tests, and arbitrary return types for supporting use
/// with the [`Termination`] trait, are also supported, applied either block-wide, or on a 
/// case-by-case basis.
///
/// Examples of arbitrarty return type usage:
/// ``` ignore
#[doc = include_str!("../tests/doctest_example_termination.rs")]
/// ```
///
/// Examples of attribute usage:
/// ``` ignore
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
