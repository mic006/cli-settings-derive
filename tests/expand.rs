//! Test macro expansion with various inputs

#[test]
pub fn pass() {
    // generate with test code locally to see #[tests]
    // (not activated by default as the output contains some local context)
    //macrotest::expand_args("tests/expand/*.rs", &["--tests"]);
    macrotest::expand("tests/expand/*.rs");
}
