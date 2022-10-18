fn bool_panic(switch: bool) {
    assert!(switch);
}

// Examples of case-by-case attribute syntax
mod case_by_case {
    use test_gen::test_gen;
    use super::*;

    test_gen! {
        bool_panic => {
            no_attrs: { (true) },
            ignore: { [ignore], (true) },
            should_panic: { [should_panic], (false) },
            ignore_should_panic: { [ignore, should_panic], (false) }
        }
    }
}

// Examples of block-wide syntax
mod block_wide {
    use test_gen::test_gen;
    use super::*;

    test_gen! {
        [ignore, should_panic],
        bool_panic => {
            no_attrs: { (false) }
        }
    }
}
