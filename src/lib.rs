#![no_std]

#[macro_export]
macro_rules! test_gen {
    //////////////////
    // Result Cases //
    //////////////////

    // Ignore All, Unit-Unit
    (ignore, $helper:expr, <_, _> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { ignore, $helper, <(), ()> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // Ignore All, Unit-User Defined
    (ignore, $helper:expr, <_, $err:ty> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { ignore, $helper, <(), $err> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // Ignore All, User Defined-Unit
    (ignore, $helper:expr, <$ok:ty, _> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { ignore, $helper, <$ok, ()> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // Ignore All, User Defined-User Defined
    (ignore, $helper:expr, <$ok:ty, $err:ty> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { ignore, $helper, Result<$ok, $err> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // Unit-Unit
    ($helper:expr, <_, _> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, <(), ()> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // Unit-User Defined
    ($helper:expr, <_, $err:ty> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, <(), $err> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // User Define-Unit
    ($helper:expr, <$ok:ty, _> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, <$ok, ()> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

    // User Defined-User Defined
    ($helper:expr, <$ok:ty, $err:ty> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, Result<$ok, $err> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };

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
