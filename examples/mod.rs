mod unittest;

mod usecases {
    pub mod vec_regular;
    pub mod vec_tuple;

    pub mod primitive_regular;
    pub mod primitive_tuple;
}
use log::info;
use unittest::setup;

fn main() {
    setup::log::configure();
    info!("Nothing to do this is just a stub, please run individual tests for specific examples");
}
