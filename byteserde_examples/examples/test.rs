// #![feature(prelude_import)]
// #[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod unittest {
    pub mod setup {
        #[allow(dead_code)]
        pub mod log {
            use std::sync::Once;
            static SETUP: Once = Once::new();
            pub fn configure() {
                SETUP
                    .call_once(|| {
                        let _ = env_logger::builder()
                            .filter_level(log::LevelFilter::Trace)
                            .try_init();
                    });
            }
        }
    }
}
use byteserde::prelude::*;
use byteserde_derive::{
    ByteDeserialize, ByteEnumFrom, ByteSerializeHeap, ByteSerializeStack,
};
use byteserde_types::char_ascii;
use log::info;
use unittest::setup;
fn enums_bind_2_tuple_auto_from() {
    setup::log::configure();
    pub struct Side(u8);
    impl From<u8> for Side {
        fn from(byte: u8) -> Self {
            Side(byte)
        }
    }
    impl From<[u8; 1]> for Side {
        fn from(bytes: [u8; 1]) -> Self {
            Side(bytes[0])
        }
    }
    impl Side {
        pub fn as_byte(&self) -> u8 {
            self.0
        }
    }
    impl std::fmt::Debug for Side {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Side")
                .field(&char::from_u32(self.0 as u32).ok_or(std::fmt::Error)?)
                .finish()
        }
    }
    impl std::fmt::Display for Side {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                format_args!(
                    "{0}", & char::from_u32(self.0 as u32).ok_or(std::fmt::Error) ?
                ),
            )
        }
    }
    // #[byteserde(from(&SideEnum))]
    enum SideEnum {
        // #[byteserde(replace(Side(b'B')))]
        Buy,
        // #[byteserde(replace(Side(b'S')))]
        Sell,
    }
}
fn main() {
    enums_bind_2_tuple_auto_from();
}
