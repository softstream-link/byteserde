 use crate::integrationtest::setup;
 #[test]
fn pass_use_cases(){
    setup::log::configure();
    let t = trybuild::TestCases::new();
    t.pass("tests/byteserde_derive/scenarios/pass_struct_regular_native_types.rs");
    t.pass("tests/byteserde_derive/scenarios/pass_struct_regular_generics.rs");
    t.pass("tests/byteserde_derive/scenarios/pass_struct_regular_length_replace.rs");
    t.pass("tests/byteserde_derive/scenarios/pass_struct_tuple_native_types.rs");
    t.pass("tests/byteserde_derive/scenarios/pass_struct_tuple_generics.rs");
    t.pass("tests/byteserde_derive/scenarios/pass_struct_tuple_length_replace.rs");
}


#[test]
fn fail_use_cases(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/byteserde_derive/scenarios/panic_enum.rs");
    t.compile_fail("tests/byteserde_derive/scenarios/panic_union.rs");
    t.compile_fail("tests/byteserde_derive/scenarios/panic_struct_unit.rs");
}
