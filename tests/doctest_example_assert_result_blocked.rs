use test_gen::test_gen;

fn square(a: u32) -> u32 {
    a.pow(2)
}

fn cube(a: u32) -> u32 {
    a.pow(3)
}

fn assert_squared(a: u32, b: u32) {
    assert_eq!(square(a), b);
}

// Second helper function required for second test block
fn assert_cubed(a: u32, b: u32) {
    assert_eq!(cube(a), b);
}

test_gen! {
    fn assert_squared => {
        two_squared: { (2, 4) },
        four_squared: { (4, 16) }
    }
}

test_gen! {
    fn assert_cubed => {
        two_cubed: { (2, 8) },
        four_cubed: { (4, 64) }
    }
}
