#[macro_export]
macro_rules! string_ascii_fixed {
    ($name:ident, $LEN:literal, $PADDING:literal, $RIGHT_ALIGN: literal ) => {
        #[derive(
            byteserde_derive::ByteSerializeStack, byteserde_derive::ByteDeserialize, PartialEq,
        )]
        pub struct $name([u8; $LEN]);
        impl From<&[u8]> for $name {
            ///  Runt time check for capacity, Takes defensively and upto `LEN`, never overflows.
            fn from(bytes: &[u8]) -> Self {
                let mut new = $name([$PADDING; $LEN]);
                let take_len = core::cmp::min($LEN, bytes.len());
                if $RIGHT_ALIGN {
                    new.0[$LEN - take_len..].copy_from_slice(&bytes[..take_len]);
                } else {
                    new.0[..take_len].copy_from_slice(&bytes[..take_len]);
                }
                new
            }
        }
        impl From<&[u8; $LEN]> for $name {
            /// Compiler time check for capacity, bytes array must be same length as [StringAsciiFixed::LEN]
            fn from(bytes: &[u8; $LEN]) -> Self {
                bytes[0..].into()
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(
                    std::any::type_name::<Self>()
                        .split("::")
                        .last()
                        .ok_or(std::fmt::Error)?,
                )
                .field(&String::from_utf8_lossy(&self.0))
                .finish()
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", String::from_utf8_lossy(&self.0))
            }
        }
    };
}

// pub(crate) use string_ascii_fixed;

#[cfg(test)]
mod test {
    use log::info;

    use crate::unittest::setup;

    // use super::*;

    string_ascii_fixed!(RightAlignedSpacePadded10, 10, b' ', true);
    string_ascii_fixed!(LeftAlignedPlusPadded10, 10, b'+', false);
    #[test]
    fn test_string_ascii_fixed() {
        setup::log::configure();    

        // from slice
        let inp_msg: RightAlignedSpacePadded10 = b"hello".as_slice().into();
        info!("inp_msg: {:?}", inp_msg);
        assert_eq!(&inp_msg.0, b"     hello");

        let imp_msg: LeftAlignedPlusPadded10 = b"hello".as_slice().into();
        info!("imp_msg: {:?}", imp_msg);
        assert_eq!(&imp_msg.0, b"hello+++++");

        // from array of exact length
        let inp_msg: RightAlignedSpacePadded10 = b"1234567890".into();
        info!("inp_msg: {:?}", inp_msg);
        assert_eq!(&inp_msg.0, b"1234567890");
    }
}
