pub trait ToLeBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

/// calling
///     impl_ToLeBytes!(i16, 2);
/// will generate
///     impl crate::utils::numerics::le_bytes::ToLeBytes<2> for i16 {
///         fn to_bytes(&self) -> [u8; 2] {
///             self.to_le_bytes()
///         }
///     }
macro_rules! impl_ToLeBytes {
    ($name:ty, $len:expr) => {
        impl $crate::utils::numerics::le_bytes::ToLeBytes<$len> for $name {
            #[inline(always)]
            fn to_bytes(&self) -> [u8; $len] {
                self.to_le_bytes()
            }
        }
    };
}

const USIZE: usize = std::mem::size_of::<usize>();
impl_ToLeBytes!(u16, 2);
impl_ToLeBytes!(i16, 2);
impl_ToLeBytes!(u32, 4);
impl_ToLeBytes!(i32, 4);
impl_ToLeBytes!(u64, 8);
impl_ToLeBytes!(i64, 8);
impl_ToLeBytes!(u128, 16);
impl_ToLeBytes!(i128, 16);
impl_ToLeBytes!(f32, 4);
impl_ToLeBytes!(f64, 8);
impl_ToLeBytes!(usize, USIZE);
impl_ToLeBytes!(isize, USIZE);
// //////////////////////////////////////////////////////////////////////
pub trait FromLeBytes<const N: usize, T> {
    fn from_bytes(v: [u8; N]) -> T;
}

/// calling
///     impl_FromLeBytes!(i16, 2);
/// will generate
///     impl crate::utils::numerics::le_bytes::FromLeBytes<2, i16> for i16 {
///         fn from_bytes(v: [u8; 2]) -> i16 {
///             i16::from_le_bytes(v)
///         }
///     }
macro_rules! impl_FromLeBytes {
    ($name:ty, $len:expr) => {
        impl $crate::utils::numerics::le_bytes::FromLeBytes<$len, $name> for $name {
            #[inline(always)]
            fn from_bytes(v: [u8; $len]) -> $name {
                <$name>::from_le_bytes(v)
            }
        }
    };
}
impl_FromLeBytes!(u8, 1);
impl_FromLeBytes!(i8, 1);
impl_FromLeBytes!(u16, 2);
impl_FromLeBytes!(i16, 2);
impl_FromLeBytes!(u32, 4);
impl_FromLeBytes!(i32, 4);
impl_FromLeBytes!(u64, 8);
impl_FromLeBytes!(i64, 8);
impl_FromLeBytes!(u128, 16);
impl_FromLeBytes!(i128, 16);
impl_FromLeBytes!(f32, 4);
impl_FromLeBytes!(f64, 8);
impl_FromLeBytes!(usize, USIZE);
impl_FromLeBytes!(isize, USIZE);

// //////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::unittest::setup;
    use log::info;
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
        assert_eq!(byt, [0x00_u8, 0xAA_u8]);
        assert_eq!(inp, out);
    }
}
