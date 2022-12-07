// Example of case-by-case arbitrary result syntax
mod case_by_case {
    use std::process::ExitCode;
    use test_gen::test_gen;

    struct Example;

    impl From<Example> for () {
        fn from(_: Example) -> Self {
            ()
        }
    }

    test_gen! {
        fn Into::into => {
            into_exit_code: {
                (0) -> ExitCode
            },
            into_unit: {
                (Example)
            },
        }
    }
}

// Example of block-wide arbitrary result syntax
mod block_wide {
    use test_gen::test_gen;

    fn clamp_result(a: u32) -> Result<(), ()> {
        (1..101).contains(&a).then_some(()).ok_or(())
    }

    test_gen! {
        fn clamp_result -> Result<(), ()> => {
            one: {
                (1)
            },
            one_hundred: {
                (100)
            },
        }
    }
}
