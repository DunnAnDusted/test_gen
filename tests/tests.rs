use std::process::{
    ExitCode,
    Termination,
};
use test_gen::test_gen;

mod unit_return {
    use super::*;

    mod unique_attrs {
        use super::*;

        test_gen! {
            bool_panic => {
                no_attr_one: { (true) },
                no_attr_two: { (true) },
                panic_one: { (false), [should_panic] },
                panic_two: { (false), [should_panic] },
                ignore_one: { (true), [ignore] },
                ignore_two: { (true), [ignore] },
                panic_ignore_one: { (false), [should_panic, ignore] },
                panic_ignore_two: { (false), [should_panic, ignore] }
            }
        }
    }

    mod all_panic {
        use super::*;

        test_gen! {
            should_panic,
            panic_button => {
                one: { (()) },
                two: { (()) },
                three: { (()), [ignore] },
                four: { (()), [ignore] }
            }
        }
    }

    mod all_ignore {
        use super::*;

        test_gen! {
            ignore,
            bool_panic => {
                one: { (true) },
                two: { (true) },
                three: { (false), [should_panic] },
                four: { (false), [should_panic] }
            }
        }
    }

    mod all_ignore_should_panic {
        use super::*;

        test_gen! {
            ignore,
            should_panic,
            panic_button => {
                one: { (()) },
                two: { (()) }
            }
        }
    }

    mod all_should_panic_ignore {
        use super::*;

        test_gen! {
            should_panic,
            ignore,
            panic_button => {
                one: { (()) },
                two: { (()) }
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
                a_ok,
                UResult => {
                    no_attrs_one: { (()) },
                    no_attrs_two: { (()) },
                    ignore_one: { (()), [ignore] },
                    ignore_two: { (()), [ignore] }
                }
            }
        }
    }

    mod into_ec {
        use super::*;

        mod unique_attrs {
            use super::*;

            test_gen! {
                Into::into,
                ExitCode => {
                    no_attrs_one: { (0) },
                    no_attrs_two: { (0) },
                    ignore_one: { (0), [ignore] },
                    ignore_two: { (0), [ignore] }
                }
            }
        }

        mod all_ignore {
            use super::*;

            test_gen! {
                ignore,
                Into::into,
                ExitCode => {
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
                    into_ec: { ExitCode, (0) },
                    into_test: { Test, (0) },
                    into_ec_ignore: { ExitCode, (0), [ignore] },
                    into_test_ignore: { Test, (0), [ignore] }
                }
            }
        }

        mod all_ignore {
            use super::*;

            test_gen! {
                ignore,
                Into::into => {
                    into_ec: { ExitCode, (0) },
                    into_test: { Test, (0) }
                }
            }
        }
    }
}

struct Test(ExitCode);

impl From<usize> for Test {
    fn from(convert: usize) -> Self {
        Test((convert as u8).into())
    }
}

impl Termination for Test {
    fn report(self) -> ExitCode {
        self.0
    }
}

fn a_ok<T, E>(pass: T) -> Result<T, E> {
    Ok(pass)
}

fn bool_panic(switch: bool) {
    assert!(switch);
}

fn panic_button<T>(_: T) {
    panic!("should panic");
}
