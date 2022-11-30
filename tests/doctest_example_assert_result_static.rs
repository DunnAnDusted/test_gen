use test_gen::test_gen;

fn square(a: u32) -> u32 {
    a.pow(2)
}

fn cube(a: u32) -> u32 {
    a.pow(3)
}

// Only requires a single helper function,
// due to leading static argument
fn assert_result<F: FnOnce(u32) -> u32>(f: F, a: u32, b: u32) {
    assert_eq!(f(a), b);
}

test_gen! {
    fn assert_result (square) => {
        two_squared: { (2, 4) },
        four_squared: { (4, 16) },
    }
}

test_gen! {
    fn assert_result (cube) => {
        two_cubed: { (2, 8) },
        four_cubed: { (4, 64) },
    }
}
