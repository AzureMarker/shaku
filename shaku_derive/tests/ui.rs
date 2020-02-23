#[test]
fn compile_fail() {
    let harness = trybuild::TestCases::new();

    harness.compile_fail("tests/ui/*.rs");
}
