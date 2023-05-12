pub trait ToBeBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

/// calling
///     impl_ToBeBytes!(i16, 2);
/// will generate
///     impl crate::utils::numerics::be_bytes::ToBeBytes<2> for i16 {
///         fn to_bytes(&self) -> [u8; 2] {
///             self.to_le_bytes()
///         }
///     }
macro_rules! impl_ToBeBytes {
    ($name:ty, $len:expr) => {
        impl $crate::utils::numerics::be_bytes::ToBeBytes<$len> for $name {
            #[inline(always)]
            fn to_bytes(&self) -> [u8; $len] {
                self.to_be_bytes()
            }
        }
    };
}

impl_ToBeBytes!(u8, 1);
impl_ToBeBytes!(i8, 1);
impl_ToBeBytes!(u16, 2);
impl_ToBeBytes!(i16, 2);
impl_ToBeBytes!(u32, 4);
impl_ToBeBytes!(i32, 4);
impl_ToBeBytes!(u64, 8);
impl_ToBeBytes!(i64, 8);
impl_ToBeBytes!(u128, 16);
impl_ToBeBytes!(i128, 16);
impl_ToBeBytes!(f32, 4);
impl_ToBeBytes!(f64, 8);
// //////////////////////////////////////////////////////////////////////

pub trait FromBeBytes<const N: usize, T> {
    fn from_bytes(v: [u8; N]) -> T;
}

/// calling
///     impl_FromBeBytes!(i16, 2);
/// will generate
///     impl crate::utils::numerics::be_bytes::FromBeBytes<2, i16> for i16 {
///         fn from_bytes(v: [u8; 2]) -> i16 {
///             i16::from_be_bytes(v)
///         }
///     }
macro_rules! impl_FromBeBytes {
    ($name:ty, $len:expr) => {
        impl $crate::utils::numerics::be_bytes::FromBeBytes<$len, $name> for $name {
            #[inline(always)]
            fn from_bytes(v: [u8; $len]) -> $name {
                <$name>::from_be_bytes(v)
            }
        }
    };
}
impl_FromBeBytes!(u8, 1);
impl_FromBeBytes!(i8, 1);
impl_FromBeBytes!(u16, 2);
impl_FromBeBytes!(i16, 2);
impl_FromBeBytes!(u32, 4);
impl_FromBeBytes!(i32, 4);
impl_FromBeBytes!(u64, 8);
impl_FromBeBytes!(i64, 8);
impl_FromBeBytes!(u128, 16);
impl_FromBeBytes!(i128, 16);
impl_FromBeBytes!(f32, 4);
impl_FromBeBytes!(f64, 8);
// //////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::unittest::setup;
    use log::info;
    #[test]
    fn test_u8() {
        setup::log::configure();
        let inp = 0x0F_u8;
        let byt = inp.to_bytes();
        let out = u8::from_bytes(byt);

        info!("inp: {inp}, inp:x {inp:#04x}, inp:b {inp:08b}");
        info!("out: {out}, out:x {out:#04x}, out:b {out:08b}");
        info!("byt:x {byt:#04x}, byt:b {byt:08b}", byt = byt[0]);
        assert_eq!(inp, out);
    }
    #[test]
    fn test_u16() {
        setup::log::configure();
        let inp = 0xAA00_u16;
        let byt = inp.to_bytes();
        let out = u16::from_bytes(byt);
        info! {"inp: {inp}"}
        info!("inp: {inp}, inp:x {inp:#06x}, inp:b {inp:016b}");
        info!("out: {out}, out:x {out:#06x}, inp:b {out:016b}");
        info!(
            "byt:x 0x{byt0:02x}{byt1:02x}, out:b {byt0:08b}{byt1:08b}",
            byt0 = byt[0],
            byt1 = byt[1]
        );
        assert_eq!(byt, [0xAA_u8, 0x00_u8]);
        assert_eq!(inp, out);
    }
}
