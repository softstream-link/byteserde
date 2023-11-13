pub trait ToNeBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

/// calling
///     impl_ToNeBytes!(i16, 2);
/// will generate
///     impl crate::utils::numerics::ne_bytes::ToNeBytes<2> for i16 {
///         fn to_bytes(&self) -> [u8; 2] {
///             self.to_ne_bytes()
///         }
///     }
macro_rules! impl_ToNeBytes {
    ($name:ty, $len:expr) => {
        impl $crate::utils::numerics::ne_bytes::ToNeBytes<$len> for $name {
            #[inline]
            fn to_bytes(&self) -> [u8; $len] {
                self.to_ne_bytes()
            }
        }
    };
}
const USIZE: usize = std::mem::size_of::<usize>();
impl_ToNeBytes!(u8, 1);
impl_ToNeBytes!(i8, 1);
impl_ToNeBytes!(u16, 2);
impl_ToNeBytes!(i16, 2);
impl_ToNeBytes!(u32, 4);
impl_ToNeBytes!(i32, 4);
impl_ToNeBytes!(u64, 8);
impl_ToNeBytes!(i64, 8);
impl_ToNeBytes!(u128, 16);
impl_ToNeBytes!(i128, 16);
impl_ToNeBytes!(f32, 4);
impl_ToNeBytes!(f64, 8);
impl_ToNeBytes!(usize, USIZE);
impl_ToNeBytes!(isize, USIZE);
// //////////////////////////////////////////////////////////////////////
pub trait FromNeBytes<const N: usize, T> {
    fn from_bytes(v: [u8; N]) -> T;
    fn from_bytes_ref(v: &[u8; N]) -> T;
}

/// calling
///     impl_FromNeBytes!(i16, 2);
/// will generate
///     impl crate::utils::numerics::ne_bytes::FromNeBytes<2, i16> for i16 {
///         fn from_bytes(v: [u8; 2]) -> i16 {
///             i16::from_ne_bytes(v)
///         }
///     }
macro_rules! impl_FromNeBytes {
    ($name:ty, $len:expr) => {
        impl $crate::utils::numerics::ne_bytes::FromNeBytes<$len, $name> for $name {
            #[inline]
            fn from_bytes(v: [u8; $len]) -> $name {
                <$name>::from_ne_bytes(v)
            }
            #[inline]
            fn from_bytes_ref(v: &[u8; $len]) -> $name {
                <$name>::from_ne_bytes(*v)
            }
        }
    };
}
impl_FromNeBytes!(u8, 1);
impl_FromNeBytes!(i8, 1);
impl_FromNeBytes!(u16, 2);
impl_FromNeBytes!(i16, 2);
impl_FromNeBytes!(u32, 4);
impl_FromNeBytes!(i32, 4);
impl_FromNeBytes!(u64, 8);
impl_FromNeBytes!(i64, 8);
impl_FromNeBytes!(u128, 16);
impl_FromNeBytes!(i128, 16);
impl_FromNeBytes!(f32, 4);
impl_FromNeBytes!(f64, 8);
impl_FromNeBytes!(usize, USIZE);
impl_FromNeBytes!(isize, USIZE);
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
        info!("byt:x 0x{byt0:02x}{byt1:02x}, out:b {byt0:08b}{byt1:08b}", byt0 = byt[0], byt1 = byt[1]);
        #[cfg(target_endian = "big")]
        assert_eq!(byt, [0xAA_u8, 0x00_u8]);

        #[cfg(target_endian = "little")]
        assert_eq!(byt, [0x00_u8, 0xAA_u8]);

        assert_eq!(inp, out);
    }
}
