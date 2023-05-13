#[test]
fn fail_use_cases() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/byteserde_derive/error_scenarios/panic_enum.rs");
    t.compile_fail("tests/byteserde_derive/error_scenarios/panic_union.rs");
    t.compile_fail("tests/byteserde_derive/error_scenarios/panic_struct_unit.rs");
}
