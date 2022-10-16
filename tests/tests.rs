use std::process::ExitCode;
use test_gen::test_gen;

mod unit_return {
    use super::*;

    mod unique_attrs {
        use super::*;

        test_gen! {
            bool_panic => {
                one_no_attrs: { (true) },
                two_no_attrs: { (true) },
                three_should_panic: { [should_panic], (false) },
                four_should_panic: { [should_panic], (false) },
                five_ignore: { [ignore], (true) },
                six_ignore: { [ignore], (true) },
                seven_should_panic_ignore: { [should_panic, ignore], (false) },
                eight_should_panic_ignore: { [ignore, should_panic], (false) }
            }
        }
    }

    mod all_panic {
        use super::*;

        test_gen! {
            [should_panic],
            bool_panic => {
                one: { (false) },
                two: { (false) },
                three_ignore: { [ignore], (false) },
                four_ignore: { [ignore], (false) }
            }
        }
    }

    mod all_ignore {
        use super::*;

        test_gen! {
            [ignore],
            bool_panic => {
                one: { (true) },
                two: { (true) },
                three_should_panic: { [should_panic], (false) },
                four_should_panic: { [should_panic], (false) }
            }
        }
    }

    mod all_ignore_should_panic {
        use super::*;

        test_gen! {
            [ignore, should_panic],
            bool_panic => {
                one: { (false) },
                two: { (false) }
            }
        }
    }

    mod all_should_panic_ignore {
        use super::*;

        test_gen! {
            [should_panic, ignore],
            bool_panic => {
                one: { (false) },
                two: { (false) }
            }
        }
    }
}

mod return_term {
    use super::*;

    mod unit_result {
        use super::*;

        type UResult = Result<(), ()>;

        mod unique_attrs {
            use super::*;

            test_gen! {
                a_ok => UResult => {
                    one_no_attrs: { (()) },
                    two_no_attrs: { (()) },
                    three_ignore: { [ignore], (()) },
                    four_ignore: { [ignore], (()) }
                }
            }
        }

        mod all_ignore {
            use super::*;

            test_gen! {
                a_ok => UResult => {
                    one: { (()) },
                    two: { (()) }
                }
            }
        }
    }

    mod into_ec {
        use super::*;

        mod unique_attrs {
            use super::*;

            test_gen! {
                Into::into => ExitCode => {
                    one_no_attrs: { (0) },
                    two_no_attrs: { (0) },
                    three_ignore: { [ignore], (0) },
                    four_ignore: { [ignore], (0)}
                }
            }
        }

        mod all_ignore {
            use super::*;

            test_gen! {
                [ignore],
                Into::into => ExitCode => {
                    one: { (0) },
                    two: { (0) }
                }
            }
        }
    }

    mod into_unique {
        use super::*;

        mod unique_attrs {
            use super::*;

            test_gen! {
                Into::into => {
                    into_ec: { (0) => ExitCode },
                    into_unit: { (()) },
                    into_ec_ignore: { [ignore], (0) => ExitCode },
                    into_unit_ignore: { [ignore], (()) }
                }
            }
        }

        mod all_ignore {
            use super::*;

            test_gen! {
                [ignore],
                Into::into => {
                    into_ec: { (0) => ExitCode },
                    into_unit: { (()) }
                }
            }
        }
    }
}

fn a_ok<T, E>(pass: T) -> Result<T, E> {
    Ok(pass)
}

fn bool_panic(switch: bool) {
    assert!(switch);
}
