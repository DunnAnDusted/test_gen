#![no_std]

#[macro_export]
macro_rules! test_gen {
    (ignore, should_panic, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { should_panic, ignore, $helper => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };
    (should_panic, ignore, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { ($($case_args),+), [should_panic, ignore $(, $($attr),+)?] }),+ } }
    };
    (should_panic, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { ($($case_args),+), [should_panic $(, $($attr),+)?] }),+ } }
    };
    (ignore, $helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { ($($case_args),+), [ignore $(, $($attr),+)?] }),+ } }
    };
    ($helper:expr, <_, _> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, <(), ()> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };
    ($helper:expr, <_, $err:ty> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, <(), $err> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };
    ($helper:expr, <$ok:ty, _> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, <$ok, ()> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };
    ($helper:expr, <$ok:ty, $err:ty> => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, Result<$ok, $err> => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };
    ($helper:expr => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper, () => { $($case_name: { ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };
    ($helper:expr, $return:ty => { $($case_name:ident: { ($($case_args:expr),+) $(, [$($attr:meta),+])? }),+ }) => {
        $crate::test_gen! { $helper => { $($case_name: { $return, ($($case_args),+) $(, [$($attr),+])? }),+ } }
    };
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
