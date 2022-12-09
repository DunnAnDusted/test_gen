# Contibuting to `test_gen`

Thanks for your interest in contributing to `test_gen`!

## Pulls Requests

Please make pull requests against the `main` branch,
unless changes have been discussed prior, as part of a contribution
to a branch with ongoing development.

All pull requests should detail, or link to a relevant issue detailing,
the reasoning behind its changes.

## Handling Tests

Where possible, changes should pass existing tests.

In the case existing test need to be updated to pass, please title the request
with \[MODIFIES TESTS\] suggested label as a draft PR, and request changes be reviewed,
to help ensure coverage of changes is comprehensive, and aligns stylistically.

Similarly, when adding new tests, these can be added as either unit,
or integration tests, and should be similarly titled, with the \[ADDS TESTS\]
suggested label, and preferably, with review requested.

## Test Naming and Documentation Tests

When looking into the `tests` directory, you may notice a number of files with the naming convention,
`doctest_example_**.rs`.

These files have been created as separate integration tests,
to ensure the code examples within the documentation are validated,
as Rusts testing framework does not run `#[test]` annotations within documentation.

Therefore, if you would like to contribute a new code example to be included in the documentation,
please observe these conventions:

- Example tests, should be committed separately to ammendments to documentation.
	- Commit any new test files in accordance with the instructions under [Handling Tests](#Handling Tests) (As a batch, or individually).
	- Commit any new sections and examples to the documentation (See include format below).
- Example tests, should be added under the `tests` directory.
- Example tests, should have a name prefixed with `doctest_example_`, followed by a name describing what they demonstrate.
- Examples should be included using the following format:
```rust
/// [PREFERED] Description of example:
/// ```no_run
#[doc = doctest_example!("example_test_name")]
/// ```
```
(Note, `doctest_example!` acts as a shorthand for `include_str!`,
meaning examples are only required to specify the descriptor section of the file path.)
