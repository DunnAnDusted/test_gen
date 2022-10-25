use test_gen::test_gen;

fn square(a: u32) -> u32 {
    a.pow(2)
}

test_gen! {
    |a, b| assert_eq!(square(a), b) => {
        two_squared: { (2, 4) },
        four_squared: { (4, 16) }
    }
}
