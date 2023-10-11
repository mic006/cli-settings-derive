//! Test macro expansion with various inputs

#[test]
pub fn pass() {
    // generate with test code
    macrotest::expand_args("tests/expand/*.rs", &["--tests"]);
}
