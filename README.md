# polars-subsecond-testing

This is an example for how to use subsecond to cut down on Polars compile time with Dioxus subsecond.

Subsecond info:
https://docs.rs/subsecond/latest/subsecond/

A Dioxus desktop application with a simple button can hot-patch functions without needing to build or run the application again.

This can be particularly useful for testing transformation and/aggregations of Polars DataFrames.

The idea is to create a seperate Dioxus component for a set of functions. Here, the component is gated behind the `test_correlation` feature.

# Example

To run this example, you will need to `dx` cli tool:
https://github.com/DioxusLabs/dioxus/blob/main/packages/cli/README.md

The test example can be run as follows:
`dx serve --hot-patch --features test_correlation`

# Caveats

This doesn't help with `maturin develop` compile times, but it should definitely help reduce iteration speed on test data.