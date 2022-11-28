use std::process::{ExitCode, Termination};
use test_gen::*;

mod unique_attrs {
    use super::*;

    test_gen! {
        fn bool_panic => {
            one_no_attrs: {
                (true)
            },
            two_should_panic: {
                #[should_panic]
                (false)
            },
            three_ignore: {
                #[ignore]
                (true)
            },
            four_should_panic_ignore: {
                #[should_panic]
                #[ignore]
                (false)
            },
        }
    }
}

mod all_ignore {
    use super::*;

    test_gen! {
        #[ignore]
        fn bool_panic => {
            one_no_attrs: {
                (true)
            },
            two_should_panic: {
                #[should_panic]
                (false)
            },
        }
    }
}

mod all_should_panic {
    use super::*;

    test_gen! {
        #[should_panic]
        fn bool_panic => {
            one_no_attrs: {
                (false)
            },
            two_ignore: {
                #[ignore]
                (false)
            },
        }
    }
}

mod unique_return {
    use super::*;

    struct Test;

    impl From<Test> for () {
        fn from(_: Test) -> Self {
            ()
        }
    }

    test_gen! {
        fn From::from => {
            one_exit_code: {
                (0) -> ExitCode
            },
            two_unit: {
                (Test)
            },
        }
    }
}

mod static_return {
    use super::*;

    test_gen! {
        fn Termination::report -> ExitCode => {
            one_exit_code: {
                (ExitCode::SUCCESS)
            },
            two_result: {
                (Result::<_, ()>::Ok(()))
            },
        }
    }
}

mod override_return {
    use super::*;
    use std::{
        convert::{Infallible, TryFrom, TryInto},
        num::TryFromIntError,
    };

    struct Test(u8);

    impl TryFrom<usize> for Test {
        type Error = TryFromIntError;

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            value.try_into().map(Self)
        }
    }

    impl Termination for Test {
        fn report(self) -> ExitCode {
            self.0.into()
        }
    }

    impl From<Test> for ExitCode {
        fn from(value: Test) -> Self {
            Self::from(value.0)
        }
    }

    test_gen! {
        fn TryFrom::try_from -> Result<ExitCode, Infallible> => {
            one_result_test: {
                (Test(0))
            },
            two_result_exit_code: {
                (0)
            },
            three_result_test: {
                (0) -> Result<Test, TryFromIntError>
            },
        }
    }
}

mod static_args {
    use super::*;
    use std::fmt::Debug;

    fn assert_result<T, U, V: Debug + PartialEq, F: FnOnce(T, U) -> V>(f: F, s: T, a: U, b: V) {
        assert_eq!(f(s, a), b);
    }

    test_gen! {
        fn assert_result (usize::pow, 2) => {
            one_two_squared: {
                (2, 4)
            },
            two_four_squared: {
                (4, 16)
            },
        }
    }

    test_gen! {
        fn assert_result (usize::abs_diff, 50) => {
            three_ad_60: {
                (60, 10)
            },
            four_ad_40: {
                (40, 10)
            },
        }
    }
}

fn bool_panic(switch: bool) {
    assert!(switch);
}
