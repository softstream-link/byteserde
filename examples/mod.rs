mod unittest;

mod usecases {
    // numerics
    pub mod numeric_regular;
    pub mod numeric_tuple;
    // strings
    pub mod strings_regular;
    pub mod strings_tuple;
    // vec
    pub mod vec_regular;
    pub mod vec_tuple;

    // generics
    pub mod generics_regular;
    pub mod generics_tuple;

    // practical
    pub mod practical_regular;
    pub mod practical_tuple;
    
}
use log::info;
use unittest::setup;

fn main() {
    setup::log::configure();
    info!("Nothing to do this is just a stub, please run individual tests for specific examples");
}
