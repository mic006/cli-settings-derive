/// Test expansion with various inputs
#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}
