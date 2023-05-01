// Lib is present to allow for benchmarks and integration tests
pub mod cli;
pub mod info;
pub mod ui;
// Provide the git repo setup function for benchmarks, integration tests and
// some unit tests via a library module to avoid code duplication.
#[cfg(feature = "test-utils")]
pub mod utils;
