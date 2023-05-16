mod unittest;

mod usecases {
    pub mod vec_regular;
    pub mod vec_tuple;
}
use log::info;
use unittest::setup;

fn main() {
    setup::log::configure();
    info!("Nothing to do this is just a stub, please run individual tests for examples");
}
