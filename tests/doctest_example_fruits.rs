use test_gen::*;

enum Fruit<T> {
    Apple,
    Pear,
    Other(T),
}

enum BrambleFruit {
    BlackBerry,
}

trait NameOf {
    fn name_of(&self) -> &str;
}

impl<T: NameOf> NameOf for Fruit<T> {
    fn name_of(&self) -> &str {
        use Fruit::*;

        match self {
            Apple => "apple",
            Pear => "pear",
            Other(fruit) => fruit.name_of(),
        }
    }
}

impl NameOf for BrambleFruit {
    fn name_of(&self) -> &str {
        use BrambleFruit::*;

        match self {
            BlackBerry => "blackberry",
        }
    }
}

// Helper function
fn assert_fruit<T: NameOf>(fruit: Fruit<T>, name: &str) {
    assert_eq!(fruit.name_of(), name);
}

// Test specification
test_gen! {
    // Normal turbofish syntax can be used,
    // when concrete type specification is required
    fn assert_fruit::<BrambleFruit> => {
        apple: {
            (Fruit::Apple, "apple")
        },
        // Applying case specific attributes
        pear: {
            #[ignore]
            (Fruit::Pear, "pear")
        },
        isnt_pear: {
            #[should_panic]
            (Fruit::Pear, "apple")
        },
        blackberry: {
            (Fruit::Other(BrambleFruit::BlackBerry), "blackberry")
        },
    }
}
