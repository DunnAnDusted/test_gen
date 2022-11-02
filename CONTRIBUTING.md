# Contibuting to `test_gen`

Thanks for your interst in contributing to `test_gen`!

## Contribution Scope

Due to the limited scope of this crate, contributions are open, primarily
for the refinement of documentation or resolve implementation errors.

To this end, the items of this crate should generally be considered stable, feature-wise.
If you'd like to suggest further features, or refinements to existing features which go beyond
addressing implementation errors, please open an issue where it can be discussed.

## Pulls Requests

Please make pull requests against the `main` branch,
unless changes have been discussed prior, as part of a contribution
to a branch with ongoing development.

All pull requests should detail, or link to a relevant issue detailing,
the reasoning behind its changes.

## Handling Tests

Where possible, changes should pass existing tests, though due to the nature
of the crate, this may tend to be unlikely.

In the case existing test need to be updated to pass, please title the request
with \[MODIFIES TESTS\] suggested label as a draft PR, and request changes be reviewed,
to help ensure coverage of changes is comprehensive, and aligns stylistically.

Similarly, when adding new tests, these can be added as either unit,
or integration tests, and should be similarly titled, with the \[ADDS TESTS\]
suggested label, and preferably, with review requested.

(Note: The `#![no_std]` attribute is applied at the crate scope of this project.

In the case a given test depends on features from the Standard Library,
it's recommended these be added as unit tests.)
