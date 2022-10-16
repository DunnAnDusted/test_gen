#![no_std]

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
