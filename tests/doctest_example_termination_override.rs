use std::{
    convert::{Infallible, TryFrom, TryInto},
    num::TryFromIntError,
    process::{ExitCode, Termination},
};
use test_gen::test_gen;

struct Example(u8);

impl TryFrom<usize> for Example {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

impl Termination for Example {
    fn report(self) -> ExitCode {
        self.0.into()
    }
}

impl From<Example> for ExitCode {
    fn from(value: Example) -> Self {
        Self::from(value.0)
    }
}

test_gen! {
    fn TryFrom::try_from -> Result<ExitCode, Infallible> => {
        one_result_test: {
            (Example(0))
        },
        two_result_exit_code: {
            (0)
        },
        three_result_test: {
            (0) -> Result<Example, TryFromIntError>
        },
    }
}
