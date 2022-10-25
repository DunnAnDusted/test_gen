// Example of case-by-case arbitrary result syntax
mod case_by_case {
    use std::process::ExitCode;
    use test_gen::test_gen;

    fn call<T, U, F: Fn(T) -> U>(f: F, t: T) -> U {
        f(t)
    }

    test_gen! {
        call => {
            into_exit_code: { (Into::into, 0) => ExitCode },
            // `test_gen` supports language features as normal,
            // including generic parameters and return values.
            passthrough_unit: { (|switch| assert!(switch), true) }
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
        clamp_result => Result<(), ()> => {
            one: { (1) },
            one_hundred: { (100) }
        }
    }
}
