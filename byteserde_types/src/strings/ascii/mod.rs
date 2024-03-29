use byteserde::error::Result;
use byteserde::prelude::*;
use byteserde::utils::hex::{to_hex_line, to_hex_pretty};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::any::type_name;
use std::cmp::min;
use std::fmt;

/// A string of ascii characters, padded with a constant byte, allocated on stack using `[u8; LEN]`
/// ```
/// use ::byteserde_types::prelude::*;
///
/// // Takes [u8; 5] array, which `exact` capacity as [StringAsciiFixed], compile time check on capacity
/// let inp_str: StringAsciiFixed<5, 0x20, false> = b"ABCDE".into();
/// println!("{:x}", inp_str);
/// assert_eq!(inp_str.bytes(), [0x41, 0x42, 0x43, 0x44, 0x45]);
///
/// // Takes `only` 5 bytes no alignment effect or padding due capacity.
/// let inp_str: StringAsciiFixed<5, 0x20, false> = b"ABCDEFG".as_slice().into();
/// println!("{:x}", inp_str);
/// assert_eq!(inp_str.bytes(), [0x41, 0x42, 0x43, 0x44, 0x45]);
///
/// // Takes `only` 5 bytes no alignment effect or padding due capacity.
/// let inp_str: StringAsciiFixed<5, 0x20, true> = b"ABCDEFG".as_slice().into();
/// println!("{:x}", inp_str);
/// assert_eq!(inp_str.bytes(), [0x41, 0x42, 0x43, 0x44, 0x45]);
///
/// // Takes `all` 4 bytes and aligns to the LEFT, while padding with SPACE
/// let inp_str: StringAsciiFixed<5, 0x20, false> = b"ABCD".as_slice().into();
/// println!("{:x}", inp_str);
/// assert_eq!(inp_str.bytes(), [0x41, 0x42, 0x43, 0x44, 0x20]);
///
/// // Takes `all` 4 bytes and aligns to the RIGHT, while padding with SPACE
/// let inp_str: StringAsciiFixed<5, 0x20, true> = b"ABCD".as_slice().into();
/// println!("{:x}", inp_str);
/// assert_eq!(inp_str.bytes(), [0x20, 0x41, 0x42, 0x43, 0x44]);
/// ```
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone)]
pub struct StringAsciiFixed<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool>([u8; LEN]);
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn bytes(&self) -> &[u8] {
        &self.0[0..]
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> Serialize for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_str())
    }
}
impl<'de, const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> Deserialize<'de> for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let ascii_str = String::deserialize(deserializer)?;
        Ok(Self::from(ascii_str.as_bytes()))
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> Default for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn default() -> Self {
        Self([PADDING; LEN])
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> From<&[u8]> for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    ///  Runt time check for capacity, Takes defensively and up to `LEN`, never overflows.
    fn from(bytes: &[u8]) -> Self {
        let mut new = StringAsciiFixed::<LEN, PADDING, RIGHT_ALIGN>([PADDING; LEN]);
        let take_len = min(LEN, bytes.len());
        if RIGHT_ALIGN {
            new.0[LEN - take_len..].copy_from_slice(&bytes[..take_len]);
        } else {
            new.0[..take_len].copy_from_slice(&bytes[..take_len]);
        }
        new
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> From<&[u8; LEN]> for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    /// Compiler time check for capacity, bytes array must be same length as `LEN`
    fn from(bytes: &[u8; LEN]) -> Self {
        bytes[0..].into()
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> From<u16> for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn from(value: u16) -> Self {
        if LEN < 5 {
            panic!("StringAsciiFixed<{LEN}, {PADDING}, {RIGHT_ALIGN}> cannot hold u16, LEN must be at least 5 bytes")
        }
        value.to_string().as_bytes().into()
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> From<u32> for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn from(value: u32) -> Self {
        if LEN < 10 {
            panic!("StringAsciiFixed<{LEN}, {PADDING}, {RIGHT_ALIGN}> cannot hold u32, LEN must be at least 10 bytes")
        }
        value.to_string().as_bytes().into()
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> From<u64> for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn from(value: u64) -> Self {
        if LEN < 20 {
            panic!("StringAsciiFixed<{LEN}, {PADDING}, {RIGHT_ALIGN}> cannot hold u64, LEN must be at least 20 bytes")
        }
        value.to_string().as_bytes().into()
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> fmt::LowerHex for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let name = type_name::<Self>().split("::").last().ok_or(fmt::Error)?.split('<').take(1).last().ok_or(fmt::Error)?;
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(&self.0)),
            false => to_hex_line(&self.0),
        };
        write!(f, "{name}<0x{LEN:02x}, 0x{PADDING:02x}, {align}>({bytes})", align = if RIGHT_ALIGN { "'R'" } else { "'L'" })
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> fmt::Display for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
impl<const LEN: usize, const PADDING: u8, const RIGHT_ALIGN: bool> fmt::Debug for StringAsciiFixed<LEN, PADDING, RIGHT_ALIGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().split("::").last().ok_or(fmt::Error)?)
            .field(&String::from_utf8_lossy(&self.0))
            .finish()
    }
}

#[cfg(test)]
mod test_string_ascii_fixed {
    use crate::prelude::*;
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use log::info;
    use serde_json::{to_string, from_str};

    #[test]
    fn test_take() {
        setup::log::configure();
        const ELEVEN: usize = 11;
        const SPACE: u8 = b' ';
        const RIGHT: bool = true;
        let inp_str: StringAsciiFixed<ELEVEN, SPACE, RIGHT> = b"0123456789".as_slice().into();
        info!("inp_str: {}", inp_str);
        info!("inp_str:x {:x}", inp_str);
        info!("inp_str:? {:?}", inp_str);

        let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
        info!("ser_stack: {:#x}", ser_stack);

        let des = &mut ByteDeserializerSlice::new(ser_stack.as_slice());

        // take half shall FAIL
        let out_err = StringAsciiFixed::<ELEVEN, SPACE, RIGHT>::byte_deserialize_take(des, ELEVEN / 2).unwrap_err();
        info!("out_err: {:?}", out_err);

        // take double shall FAIL
        let out_err = StringAsciiFixed::<ELEVEN, SPACE, RIGHT>::byte_deserialize_take(des, ELEVEN * 2).unwrap_err();
        info!("out_err: {:?}", out_err);
        // take correct shall PASS - IMPORTANT no bytes depleted by failed takes
        let out_str = StringAsciiFixed::<ELEVEN, SPACE, RIGHT>::byte_deserialize(des).unwrap();
        info!("out_str: {:?}", out_str);
    }

    #[test]
    fn test_from_u64_pass() {
        setup::log::configure();
        let inp_str: StringAsciiFixed<20, b'0', true> = u64::MAX.into();
        info!("inp_str:? {:?}", inp_str)
    }
    #[test]
    #[should_panic]
    fn test_from_u64_fail() {
        let _: StringAsciiFixed<19, b'0', true> = u64::MAX.into();
    }
    #[test]
    fn test_from_u32_pass() {
        setup::log::configure();
        let inp_str: StringAsciiFixed<10, b'0', true> = u32::MAX.into();
        info!("inp_str:? {:?}", inp_str)
    }
    #[test]
    #[should_panic]
    fn test_from_u32_fail() {
        let _: StringAsciiFixed<9, b'0', true> = u32::MAX.into();
    }
    #[test]
    fn test_from_u16_pass() {
        setup::log::configure();
        let inp_str: StringAsciiFixed<5, b'0', true> = u16::MAX.into();
        info!("inp_str:? {:?}", inp_str)
    }
    #[test]
    #[should_panic]
    fn test_from_u16_fail() {
        let _: StringAsciiFixed<4, b'0', true> = u16::MAX.into();
    }
    #[test]
    fn test_json(){
        setup::log::configure();
        let inp_str: StringAsciiFixed<5, b'0', true> = 12345_u16.into();
        info!("inp_str:? {:?}", inp_str);
        let out_json = to_string(&inp_str).unwrap();
        info!("out_json: {}", out_json);
        assert_eq!(out_json, r#""12345""#);
        let out_str: StringAsciiFixed<5, b'0', true> = from_str(&out_json).unwrap();
        info!("out_str:? {:?}", out_str);
        assert_eq!(out_str, inp_str);
    }
}

/// A string of ascii characters with a variable length allocated on heap using `Vec<u8>`
///
/// ```
/// use ::byteserde_types::prelude::*;
/// use ::byteserde::prelude::*;
/// use ::serde::de::*;
///
/// // Take all bytes from array
/// let inp_str: StringAscii = b"ABCDE".into();
/// println!("inp_str: {:x}", inp_str);
/// assert_eq!(inp_str.bytes(), [0x41, 0x42, 0x43, 0x44, 0x45]);
///
/// // Take all bytes from slice
/// let inp_str: StringAscii = b"ABCDEFG".as_slice().into();
/// println!("inp_str: {:x}", inp_str);
/// assert_eq!(inp_str.bytes(), [0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47]);
///
/// // Serialize and deserialize
/// let inp_str: StringAscii = b"ABCDE".into();
/// println!("inp_str: {:x}", inp_str);
/// // serialize TWICE
/// let mut ser_stack: ByteSerializerStack<128> =  to_serializer_stack(&inp_str).unwrap();
/// ser_stack.serialize(&inp_str).unwrap();
/// println!("ser_stack: {:#x}", ser_stack);
/// // deserialize NOTE - This completely DEPLETES entire buffer instead of just only once for the original string
/// let out_str: StringAscii = from_serializer_stack(&ser_stack).unwrap();
/// println!("out_str: {:x}", out_str);
/// assert_eq!(StringAscii::from(b"ABCDEABCDE"), out_str);
/// ```
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone)]
pub struct StringAscii(Vec<u8>);
impl StringAscii {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn bytes(&self) -> &[u8] {
        &self.0[0..]
    }
}
impl From<&[u8]> for StringAscii {
    fn from(bytes: &[u8]) -> Self {
        Self(Vec::<u8>::from(bytes))
    }
}
impl<const LEN: usize> From<&[u8; LEN]> for StringAscii {
    fn from(bytes: &[u8; LEN]) -> Self {
        Self(bytes[0..].into())
    }
}
impl fmt::LowerHex for StringAscii {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let name = type_name::<Self>().split("::").last().ok_or(fmt::Error)?;
        let bytes = match f.alternate() {
            true => format!("\n{hex}", hex = to_hex_pretty(&self.0)),
            false => to_hex_line(&self.0),
        };
        write!(f, "{name}({bytes})")
    }
}
impl fmt::Debug for StringAscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().split("::").last().ok_or(fmt::Error)?)
            .field(&String::from_utf8_lossy(&self.0))
            .finish()
    }
}
impl fmt::Display for StringAscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
impl serde::Serialize for StringAscii {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        String::from_utf8_lossy(&self.0).serialize(serializer)
    }
}
impl<'de> serde::Deserialize<'de> for StringAscii {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ascii_str = String::deserialize(deserializer)?;
        Ok(Self::from(ascii_str.as_bytes()))
    }
}

#[cfg(test)]
mod test_string_ascii {
    use crate::prelude::*;
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use log::info;

    #[test]
    fn test_string_ascii_take() {
        setup::log::configure();
        let inp_str: StringAscii = b"0123456789".into();
        info!("inp_str: {:x}", inp_str);

        let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_str).unwrap();
        info!("ser_stack: {:#x}", ser_stack);

        let des = &mut ByteDeserializerSlice::new(ser_stack.as_slice());

        // take half + 1 shall SUCCESS first time
        let depleted = inp_str.len() / 2 + 1;
        let out_str = StringAscii::byte_deserialize_take(des, depleted).unwrap();
        info!("out_str: {:x}", out_str);
        assert_eq!(out_str.bytes(), &inp_str.bytes()[0..depleted]);

        // take half + 1 shall FAILS second  time
        let out_err = StringAscii::byte_deserialize_take(des, depleted).unwrap_err();
        info!("out_err: {:?}", out_err);
        assert_eq!(des.remaining(), inp_str.len() - depleted);

        // take remaining shall SUCCESS third time
        let out_str = StringAscii::byte_deserialize(des).unwrap();
        info!("out_str: {:x}", out_str);
        assert_eq!(out_str.bytes(), &inp_str.bytes()[depleted..]);
        assert_eq!(des.remaining(), 0);
    }
}

/// an ascii character
/// ```
/// use ::byteserde_types::prelude::*;
///
/// let inp_char: CharAscii = b'A'.into();
/// println!("{:x}", inp_char);
/// assert_eq!(inp_char.bytes(), [0x41]);
///
/// let inp_char: CharAscii = b"AB".as_slice().into();
/// println!("{:x}", inp_char);
/// assert_eq!(inp_char.bytes(), [0x41]);
/// ```
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Copy)]
pub struct CharAscii(u8);
impl CharAscii {
    pub fn bytes(&self) -> [u8; 1] {
        [self.0]
    }
    pub fn new(byte: u8) -> Self {
        Self(byte)
    }
    pub fn as_byte(&self) -> u8 {
        self.0
    }
}
impl From<&CharAscii> for u8 {
    fn from(value: &CharAscii) -> Self {
        value.0
    }
}
impl From<u8> for CharAscii {
    fn from(value: u8) -> Self {
        CharAscii(value)
    }
}
impl From<&[u8]> for CharAscii {
    fn from(value: &[u8]) -> Self {
        CharAscii(value[0])
    }
}
impl fmt::LowerHex for CharAscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = type_name::<Self>().split("::").last().ok_or(fmt::Error)?;
        let byte = self.0;
        write!(f, "{name}(0x{byte:x})")
    }
}
impl fmt::Debug for CharAscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().split("::").last().ok_or(fmt::Error)?)
            .field(&char::from_u32(self.0 as u32).ok_or(fmt::Error)?)
            .finish()
    }
}
impl fmt::Display for CharAscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &char::from_u32(self.0 as u32).ok_or(fmt::Error)?)
    }
}

#[cfg(test)]
mod test_char_ascii {
    use crate::prelude::*;
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use log::info;

    #[test]
    fn test_char_ascii() {
        setup::log::configure();
        let inp_char: CharAscii = b'A'.into();
        info!("inp_char: {:?}", inp_char);

        let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_char).unwrap();
        info!("ser_stack: {:#x}", ser_stack);

        let ser_heap: ByteSerializerHeap = to_serializer_heap(&inp_char).unwrap();
        info!("ser_heap: {:#x}", ser_heap);
        assert_eq!(ser_stack.as_slice(), ser_heap.as_slice());

        let des = &mut ByteDeserializerSlice::new(ser_stack.as_slice());
        let out_char = CharAscii::byte_deserialize(des).unwrap();
        info!("out_char: {:?}", out_char);
    }
}

/// an ascii const character
/// ```
/// use ::byteserde_types::prelude::*;
///
/// let inp_char: ConstCharAscii<b'+'> = Default::default();
/// println!("{:x}", inp_char);
/// assert_eq!(inp_char.bytes(), [43]);
///
/// ```
#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteSerializeHeap, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone,)]
pub struct ConstCharAscii<const CHAR: u8>(u8);
impl<const CHAR: u8> ConstCharAscii<CHAR> {
    pub fn bytes(&self) -> [u8; 1] {
        [CHAR]
    }
    pub fn to_char() -> char {
        char::from_u32(u32::from(CHAR)).unwrap()
    }
}
impl<const CHAR: u8> ByteDeserializeSlice<ConstCharAscii<CHAR>> for ConstCharAscii<CHAR> {
    #[allow(clippy::just_underscores_and_digits)]
    fn byte_deserialize(des: &mut ByteDeserializerSlice) -> Result<ConstCharAscii<CHAR>> {
        let _0 = des.deserialize_u8()?;
        match _0 == CHAR {
            true => Ok(Default::default()),
            false => {
                let ty: ConstCharAscii<CHAR> = Default::default();

                Err(SerDesError {
                    message: format!("Type {:?} expected: 0x{:02x} actual: 0x{:02x}", ty, CHAR, _0),
                })
            }
        }
    }
}
impl<const CHAR: u8> Serialize for ConstCharAscii<CHAR> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_str())
    }
}
impl<'de, const CHAR: u8> Deserialize<'de> for ConstCharAscii<CHAR> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let c = char::deserialize(deserializer)?;
        match c == Self::to_char() {
            true => Ok(Default::default()),
            false => Err(serde::de::Error::custom(format!("Type {:?} expected: 0x{:02x} actual: 0x{:02x}", type_name::<Self>(), CHAR, c as u8))),
        }
    }
}
impl<const CHAR: u8> Default for ConstCharAscii<CHAR> {
    fn default() -> Self {
        Self(CHAR)
    }
}
impl<const CHAR: u8> fmt::LowerHex for ConstCharAscii<CHAR> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = type_name::<Self>().split("::").last().ok_or(fmt::Error)?;
        let byte = self.0;
        write!(f, "{name}(0x{byte:x})")
    }
}
impl<const CHAR: u8> fmt::Debug for ConstCharAscii<CHAR> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().split("::").last().ok_or(fmt::Error)?)
            .field(&char::from_u32(u32::from(self.0)).ok_or(fmt::Error)?)
            .finish()
    }
}
impl<const CHAR: u8> fmt::Display for ConstCharAscii<CHAR> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &char::from_u32(u32::from(self.0)).ok_or(fmt::Error)?)
    }
}

#[cfg(test)]
mod test_const_char_ascii {
    use crate::prelude::*;
    use crate::unittest::setup;
    use byteserde::prelude::*;
    use byteserde_derive::ByteSerializeStack;
    use log::info;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_const_char_ascii() {
        setup::log::configure();
        #[derive(ByteSerializeStack)]
        struct Values(u8, u8);
        let inp_bytes = Values(b'+', b'-');
        let ser_stack: ByteSerializerStack<128> = to_serializer_stack(&inp_bytes).unwrap();
        info!("ser_stack: {:#x}", ser_stack);

        let des = &mut ByteDeserializerSlice::new(ser_stack.as_slice());
        let out_plus: ConstCharAscii<b'+'> = des.deserialize().unwrap();
        info!("out_plus: {}", out_plus);

        let out_res: byteserde::error::Result<ConstCharAscii<b'+'>> = des.deserialize();
        info!("out_res: {:?}", out_res);
        assert!(out_res.is_err());

        let out_json = to_string(&out_plus).unwrap();
        info!("out_json: {}", out_json);
        assert_eq!(out_json, r#""+""#);
        let out_plus: ConstCharAscii<b'+'> = from_str(&out_json).unwrap();
        info!("out_plus: {}", out_plus);
        assert_eq!(out_plus, ConstCharAscii::<b'+'>::default());
    }
}
