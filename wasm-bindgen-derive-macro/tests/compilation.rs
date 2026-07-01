// This is an integration test since we need to use additional crates compared to the main library.

#[test]
fn compilation() {
    let t = trybuild::TestCases::new();
    t.pass("tests-snippets/allowed-uses.rs");
    t.compile_fail("tests-snippets/derive-on-enum.rs");
    t.compile_fail("tests-snippets/no-annotation.rs");
    t.compile_fail("tests-snippets/invalid-nested-attr.rs");
    t.compile_fail("tests-snippets/non-string-nested-attr.rs");
}
