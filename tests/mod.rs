mod integrationtest;
#[cfg(test)]
mod byteserde_derive {
    mod usecases_test;

    mod primitive_regular;
    mod primitive_tuple;

    mod generics_regular;
    mod generics_tuple;

    mod length_replace_regular;
    mod length_replace_tuple;
}
