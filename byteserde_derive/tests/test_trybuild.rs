#[test]
fn fail_use_cases() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/error_scenarios/size_of_vec.rs");
    t.compile_fail("tests/error_scenarios/union.rs");
    t.compile_fail("tests/error_scenarios/struct_unit.rs");
    t.compile_fail("tests/error_scenarios/option.rs");
}
