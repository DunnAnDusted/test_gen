# test_gen
A comprehensive function-like macro, for concisely defining parameterized tests.

This crate provides the function-like macro of its namesake, `test_gen`,
which enables the concise definition of batches of named tests,
implementing a parameterized argument format to minimise the boilerplate
otherwise required for specifying batches of similar tests.

Documentation can be found at [Docs.rs].

[Docs.rs]: https://docs.rs/test_gen/latest/test_gen

## Usage
**Minimum Supported Rust Version:** 1.63.0

`test_gen` can be added to a project, using the following command:
```text
cargo add test_gen --dev
```

Or alternatively, by adding the following lines to its `Cargo.toml` file:
```toml
[dev-dependancies]
test_gen = "0.2.2"
```

## Examples

Fruits:
```rust
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
```

## License

`test_gen` is licensed under the BSD 3-Clause license.

See [LICENSE](LICENSE) for details.
