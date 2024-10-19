#[test]
fn pass() {
    trybuild::TestCases::new().pass("./src/tests/ui/pass/*.rs");
}

#[test]
fn compile_fail() {
    trybuild::TestCases::new().compile_fail("./src/tests/ui/compile_fail/*.rs");
}
