#![no_std]

#[macro_export]
macro_rules! test_gen {
    ////////////////////
    // No/Unit Return //
    ////////////////////

    // All Ignore, All Should Panic
    (ignore, should_panic, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { should_panic, ignore, $helper => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // All Should Panic, All Ignore
    (should_panic, ignore, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { ignore, $helper, () => { $($case_name: { ($($case_args),+), [should_panic $(, $($attr),+)?] }),+ } }
    };

    // All Should Panic
    (should_panic, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { ($($case_args),+), [should_panic $(, $($attr),+)?] }),+ } }
    };

    // All Ignore
    (ignore, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { ignore, $helper, () => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // None
    ($helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, () => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    ////////////////
    // All Return //
    ////////////////

    // All Ignore
    (ignore, $helper:expr, $return:ty => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { $return, ($($case_args),+), [ignore $(, $($attr),+)?] }),+ } }
    };

    // None
    ($helper:expr, $return:ty => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { $return, ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    ///////////////////
    // Unique Return //
    ///////////////////

    // All Ignore
    (ignore, $helper:expr => { $($case_name:ident: { $return:ty, ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { $return, ($($case_args),+), [ignore $(, $($attr),+)?] }),+ } }
    };

    // BASE
    ($helper:expr => { $($case_name:ident: { $return:ty, ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $(
            $($(#[$attr])+)?
            #[test]
            fn $case_name() -> $return {
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
            unit: { (), (()) },
            result: { Result<(), ()>, (Ok(())) }
        }
    }
}
