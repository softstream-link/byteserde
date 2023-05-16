mod integrationtest;
#[cfg(test)]
mod byteserde_derive {
    mod usecases_test;

    mod generics_regular;
    mod generics_tuple;

    mod length_replace_regular;
    mod length_replace_tuple;
}
