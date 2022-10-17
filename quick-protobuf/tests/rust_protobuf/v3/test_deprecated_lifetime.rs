#[test]
fn test_owned() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/rust_protobuf/v3/test_deprecated_lifetime_must_compile_error.rs");
}