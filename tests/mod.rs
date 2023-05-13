mod integrationtest;
#[cfg(test)]
mod byteserde_derive {
    mod usecases_test;
    
    mod struct_regular_native_types;
    mod struct_tuple_native_types;
    
    mod struct_regular_generics;
    mod struct_tuple_generics;
    
    mod struct_regular_length_replace;
    mod struct_tuple_length_replace;
}


