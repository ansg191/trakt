#[test]
fn request_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/request/*.rs");
}
