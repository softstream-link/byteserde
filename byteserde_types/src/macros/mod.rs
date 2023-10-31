/// Generates a `tuple` `struct` with a given name for managing an ascii string of fixed length. Buffer is `stack` allocated using `[u8; LEN]`.
/// Macro argument signature is as follows:
///
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `LEN` - length of the buffer that will contain ascii string in bytes
/// * `PADDING` - padding byte to be used when creating string
/// * `RIGHT_ALIGN` - boolean flag to indicate if string should be right aligned or left aligned
/// * `derive(...)` -- list of traits to derive for the struct, must be valid rust traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * [std::fmt::Debug] & [std::fmt::Display] - provides a human readable sting view of the `[u8; LEN]` byte buffer as a utf-8 string
/// * [serde::Serialize] & [serde::Deserialize] - provides json style serialization of the internal byte array representing ascii from & into [String]
///
/// # From
/// Note that provided implementation already includes the following `From` implementations.
/// * `From<&[u8]>` - will take up to `LEN` bytes from the slice and pad if necessary using `PADDING` argument.
/// * `From<&[u8; LEN]>` - must match the length of the buffer, will not pad.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// // Align=Left / Len=10 / Padding=Minus
/// string_ascii_fixed!(Password, 10, b'-', false);
/// let inp_pwd = Password::new(*b"1234567890");
///
/// let inp_pwd: Password = b"12345".as_slice().into(); // from slice
/// println!("inp_pwd: {:?}, {}", inp_pwd, inp_pwd);
/// assert_eq!(inp_pwd.value(), b"12345-----");
///
/// // Align=Right / Len=10 / Padding=Space
/// string_ascii_fixed!(Username, 10, b' ', true, derive(PartialEq));
/// let inp_usr1: Username = b"12345".as_slice().into(); // from slice
/// let inp_usr2: Username = b"     12345".into(); // from array of matching len
/// println!("inp_usr1: {:?}, {}", inp_usr1, inp_usr1);
/// println!("inp_usr2: {:?}, {}", inp_usr2, inp_usr2);
/// assert_eq!(inp_usr1.value(), b"     12345");
/// assert_eq!(inp_usr2.value(), b"     12345");
/// assert_eq!(inp_usr1, inp_usr2);
/// # }
/// ```
#[macro_export]
macro_rules! string_ascii_fixed {
    ($NAME:ident, $LEN:literal, $PADDING:literal, $RIGHT_ALIGN: literal ) => {
        pub struct $NAME([u8; $LEN]);
        $crate::_common_string_ascii_fixed!($NAME, $LEN, $PADDING, $RIGHT_ALIGN);
    };
    ($NAME:ident, $LEN:literal, $PADDING:literal, $RIGHT_ALIGN: literal, $STRUCT_META:meta ) => {
        #[$STRUCT_META]
        pub struct $NAME([u8; $LEN]);
        $crate::_common_string_ascii_fixed!($NAME, $LEN, $PADDING, $RIGHT_ALIGN);
    };
}
#[macro_export]
macro_rules! _common_string_ascii_fixed {
    ($NAME:ident, $LEN:literal, $PADDING:literal, $RIGHT_ALIGN:literal ) => {
        impl $NAME {
            pub fn value(&self) -> &[u8; $LEN] {
                &self.0
            }
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
            pub fn new(value: [u8; $LEN]) -> Self {
                $NAME(value)
            }
        }
        impl serde::Serialize for $NAME {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where S: serde::Serializer {
                String::from_utf8_lossy(&self.0).trim().serialize(serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for $NAME {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where D: serde::Deserializer<'de> {
                let ascii_str = String::deserialize(deserializer)?;
                if ascii_str.len() <= $LEN {
                    Ok($NAME::from(ascii_str.as_bytes()))
                } else {
                    let msg = format!(
                        "{} being constructed from '{}' whose byte length: {} exceeds max allowed byte length: {} of the tuple struct",
                        stringify!($NAME),
                        ascii_str,
                        ascii_str.len(),
                        $LEN
                    );
                    Err(serde::de::Error::custom(msg))
                }
            }
        }
        impl From<&[u8]> for $NAME {
            ///  Runt time check for capacity, Takes defensively and up to `LEN`, never overflows.
            fn from(bytes: &[u8]) -> Self {
                let mut new = $NAME([$PADDING; $LEN]);
                let take_len = core::cmp::min($LEN, bytes.len());
                if $RIGHT_ALIGN {
                    new.0[$LEN - take_len..].copy_from_slice(&bytes[..take_len]);
                } else {
                    new.0[..take_len].copy_from_slice(&bytes[..take_len]);
                }
                new
            }
        }
        impl From<&[u8; $LEN]> for $NAME {
            /// Compiler time check for capacity, bytes array must be same length as [StringAsciiFixed::LEN]
            fn from(bytes: &[u8; $LEN]) -> Self {
                bytes[0..].into()
            }
        }
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($NAME)).field(&String::from_utf8_lossy(&self.0)).finish()
            }
        }
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", String::from_utf8_lossy(&self.0))
            }
        }
    };
}

/// Generates a `tuple` `struct` with a given name for managing an ascii char allocated on `stack` using `u8`.
///
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `derive(..)` -- list of traits to derive for the struct, must be valid rust traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * [std::fmt::Debug] & [std::fmt::Display] - provides a human readable sting view of the `u8` byte as utf-8 char
/// * [serde::Serialize] & [serde::Deserialize] - provides json style serialization of the internal byte array representing ascii from & into [String] with one char
///
/// # From
/// Note that provided implementation already includes the following `From` implementations.
/// * `From<u8>` - will take the `u8` byte and return tuple struct with type of `NAME` argument.
/// * `From<[u8; 1]>` - will take the first byte of the array and return tuple struct with type of `NAME` argument.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// char_ascii!(Char, derive(PartialEq));
/// let inp_char: Char = b'1'.into(); // from u8
/// println!("inp_char: {:?}, {}", inp_char, inp_char);
/// assert_eq!(inp_char.value(), b'1');
///
/// let inp_char: Char = [b'1'].into(); // from array
/// println!("inp_char: {:?}, {}", inp_char, inp_char);
/// assert_eq!(inp_char.value(), b'1');
/// # }
/// ```
#[macro_export]
macro_rules! char_ascii {
    ($NAME:ident) => {
        /// Tuple struct with a `u8` buffer to represent an ascii char.
        pub struct $NAME(u8);
        $crate::_common_char_ascii!($NAME);
    };
    ($NAME:ident, $STRUCT_META:meta) => {
        /// Tuple struct with a `u8` buffer to represent an ascii char.
        #[$STRUCT_META]
        pub struct $NAME(u8);
        $crate::_common_char_ascii!($NAME);
    };
}
#[macro_export]
macro_rules! _common_char_ascii {
    ($NAME:ident) => {
        impl $NAME {
            /// proves access to the `u8` byte
            pub fn value(&self) -> u8 {
                self.0
            }
            pub fn new(value: u8) -> Self {
                $NAME(value)
            }
        }
        impl serde::Serialize for $NAME {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where S: serde::Serializer {
                String::from_utf8_lossy(&[self.0]).serialize(serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for $NAME {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where D: serde::Deserializer<'de> {
                let ascii_str = String::deserialize(deserializer)?;
                if ascii_str.len() == 1 {
                    Ok($NAME::from(ascii_str.as_bytes()[0]))
                } else {
                    let msg = format!(
                        "{} being constructed from '{}' whose byte length: {} exceeds max allowed byte length: 1 of the tuple struct",
                        stringify!($NAME),
                        ascii_str,
                        ascii_str.len(),
                    );
                    Err(serde::de::Error::custom(msg))
                }
            }
        }
        impl From<u8> for $NAME {
            fn from(byte: u8) -> Self {
                $NAME(byte)
            }
        }
        impl From<[u8; 1]> for $NAME {
            fn from(bytes: [u8; 1]) -> Self {
                $NAME(bytes[0])
            }
        }
        // utf8 `char` based impl
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($NAME)).field(&char::from_u32(self.0 as u32).ok_or(std::fmt::Error)?).finish()
            }
        }
        /// utf8 `char` based impl
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &char::from_u32(self.0 as u32).ok_or(std::fmt::Error)?)
            }
        }
    };
}

/// Generates a `tuple` `struct` with a given name and a private ascii char allocated on `stack` using `u8` whose
/// value always set to parameter `CONST`.
///
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `CONST` - `u8` byte value to be used as the value behind this struct
/// * `derive(...)` -- list of traits to derive for the struct, must be valid rust traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * [std::fmt::Debug] & [std::fmt::Display] - provides a human readable sting view of the `u8` byte as utf-8 char
/// * [byteserde::prelude::ByteDeserializeSlice]- provides an implementation for deserializing from a byte stream, which returns [byteserde::prelude::SerDesError] if value on the
/// * [serde::Serialize] & [serde::Deserialize] - provides json style serialization of the internal CONST representing ascii from & into [String] with one char
/// stream does `not` match the `CONST` value.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// use byteserde::prelude::*;
/// const_char_ascii!(One, b'1', derive(PartialEq));
/// let inp_const = One::default();
/// println!("inp_const: {:?}, {}", inp_const, inp_const);
/// assert_eq!(inp_const.value(), b'1');
/// # }
/// ```
#[macro_export]
macro_rules! const_char_ascii {
    ($NAME:ident, $CONST:literal) => {
        pub struct $NAME(u8);
        $crate::_common_const_char_ascii!($NAME, $CONST);
    };
    ($NAME:ident, $CONST:literal, $STRUCT_META:meta) => {
        #[$STRUCT_META]
        pub struct $NAME(u8);
        $crate::_common_const_char_ascii!($NAME, $CONST);
    };
}
#[macro_export]
macro_rules! _common_const_char_ascii {
    ($NAME:ident, $CONST:literal) => {
        impl $NAME {
            pub fn to_char() -> char {
                char::from_u32($CONST as u32).ok_or(std::fmt::Error).unwrap()
            }
            pub fn value(&self) -> u8 {
                self.0
            }
            pub fn as_slice() -> &'static [u8] {
                &[$CONST]
            }
        }
        impl Default for $NAME {
            #[inline(always)]
            fn default() -> Self {
                $NAME($CONST)
            }
        }
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($NAME)).field(&char::from_u32(u32::from($CONST)).ok_or(std::fmt::Error)?).finish()
            }
        }
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &char::from_u32(u32::from($CONST)).ok_or(std::fmt::Error)?)
            }
        }
        impl serde::Serialize for $NAME {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where S: serde::Serializer {
                String::from_utf8_lossy(&[self.0]).serialize(serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for $NAME {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where D: serde::Deserializer<'de> {
                let ascii_str = String::deserialize(deserializer)?;
                if ascii_str == Self::to_char().to_string() {
                    Ok($NAME::default())
                } else {
                    let msg = format!(
                        "{} being constructed from '{}' whose value does not match expected const: '{}' of the tuple struct",
                        stringify!($NAME),
                        ascii_str,
                        Self::to_char(),
                    );
                    Err(serde::de::Error::custom(msg))
                }
            }
        }
        impl ::byteserde::prelude::ByteDeserializeSlice<$NAME> for $NAME {
            #[allow(clippy::just_underscores_and_digits)]
            fn byte_deserialize(des: &mut ::byteserde::prelude::ByteDeserializerSlice) -> ::byteserde::error::Result<$NAME> {
                let _0 = des.deserialize_u8()?;
                match _0 == $CONST {
                    true => Ok($NAME::default()),
                    false => {
                        let ty = $NAME::default();

                        Err(::byteserde::prelude::SerDesError {
                            message: format!("Type {:?} expected: 0x{:02x} actual: 0x{:02x}", ty, $CONST, _0),
                        })
                    }
                }
            }
        }
        impl ::byteserde::des_bytes::ByteDeserializeBytes<$NAME> for $NAME {
            #[allow(clippy::just_underscores_and_digits)]
            fn byte_deserialize(des: &mut ::byteserde::prelude::ByteDeserializerBytes) -> ::byteserde::error::Result<$NAME> {
                let _0 = des.deserialize_u8()?;
                match _0 == $CONST {
                    true => Ok($NAME::default()),
                    false => {
                        let ty = $NAME::default();

                        Err(::byteserde::prelude::SerDesError {
                            message: format!("Type {:?} expected: 0x{:02x} actual: 0x{:02x}", ty, $CONST, _0),
                        })
                    }
                }
            }
        }
    };
}

/// This is a short hand macro for generating a new `CONST` `tuple` `struct` type for u8, i8
/// Typically will not be used directly but instead will be called via one of the other macros like `const_u8_tuple`, `const_i8_tuple`
#[macro_export]
macro_rules! const_byte {
    ($NAME:ident, $CONST:literal, $TYPE:ty, $STRUCT_META:meta ) => {
        #[$STRUCT_META]
        pub struct $NAME($TYPE);
        impl $NAME {
            #[inline(always)]
            pub fn value(&self) -> $TYPE {
                self.0
            }
        }
        impl Default for $NAME {
            #[inline(always)]
            fn default() -> Self {
                $NAME($CONST)
            }
        }
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    &self.0
                )
            }
        }
    };
}
/// Generates a `CONST` `tuple` `struct` with a given name for managing a Numeric type `u8` allocated on `stack`.
///
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `CONST` - `u8` byte value to be used as the value behind this struct
/// * `derive(...)` -- list of traits to derive for the struct, must be valid rust traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * `Display` - provides a human readable sting view of the `u8` value
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// use byteserde_derive::ByteSerializeStack;
/// const_u8_tuple!(Number, 1, derive(ByteSerializeStack, PartialEq, Debug));
///
/// let inp_num = Number::default();
/// println!("inp_num: {:?}, {}", inp_num, inp_num);
/// assert_eq!(inp_num.value(), 1_u8);
/// # }
/// ```
#[macro_export]
macro_rules! const_u8_tuple {
    ($NAME:ident, $CONST:literal, $STRUCT_META:meta) => {
        $crate::const_byte!($NAME, $CONST, u8, $STRUCT_META );
    };
}
/// see [const_u8_tuple] for more details and examples.
#[macro_export]
macro_rules! const_i8_tuple {
    ($NAME:ident, $CONST:literal, $STRUCT_META:meta) => {
        $crate::const_byte!($NAME, $CONST, i8, $STRUCT_META );
    };
}

/// This is a short hand macro for generating a new `CONST` `tuple` `struct` type for numerics like u16, i16, u32, i32, u64, i64, ...
/// Typically will not be used directly but instead will be called via one of the other macros like `const_u16_tuple`, `const_i16_tuple`, ...
#[macro_export]
macro_rules! const_numeric {
    ($NAME:ident, $CONST:literal, $TYPE:ty, $ENDIAN:literal, $STRUCT_META:meta ) => {
        #[$STRUCT_META]
        #[byteserde(endian = $ENDIAN )]
        pub struct $NAME($TYPE);
        impl $NAME {
            #[inline(always)]
            pub fn value(&self) -> $TYPE {
                self.0
            }
        }
        impl Default for $NAME {
            #[inline(always)]
            fn default() -> Self {
                $NAME($CONST)
            }
        }
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    &self.0
                )
            }
        }
    };
}
/// Generates a `CONST` `tuple` `struct` with a given name for managing a Numeric type `u16` allocated on `stack`.
///
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `CONST` - `u16` byte value to be used as the value behind this struct
/// * `ENDIAN` - endianess of the numeric type, must be either `le`, `be`, or `ne`, this will be passed directly to the `byteserde` attribute as #[byteserde(endian = "xx" )]
/// * `derive(...)` -- `must include one of` the following `ByteSerializeStack`, `ByteSerializeHeap`, or `ByteDeserializeSlice` other wise the `#[byteserde(endian = $ENDIAN)]` attribute will fail to compile.
/// Plus list of additional valid rust derive traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * `Display` - provides a human readable sting view of the `u16` value
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// use byteserde_derive::ByteSerializeStack;
/// const_u16_tuple!(Number, 1, "be", derive(ByteSerializeStack, PartialEq, Debug));
///
/// let inp_num = Number::default();
/// println!("inp_num: {:?}, {}", inp_num, inp_num);
/// assert_eq!(inp_num.value(), 1_u16);
/// # }
/// ```
#[macro_export]
macro_rules! const_u16_tuple {
    ($NAME:ident, $CONST:literal, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::const_numeric!($NAME, $CONST, u16, $ENDIAN, $STRUCT_META );
    };
}

/// see [const_u16_tuple] for more details and examples.
#[macro_export]
macro_rules! const_i16_tuple {
    ($NAME:ident, $CONST:literal, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::const_numeric!($NAME, $CONST, i16, $ENDIAN, $STRUCT_META );
    };
}

/// see [const_u16_tuple] for more details and examples.
#[macro_export]
macro_rules! const_u32_tuple {
    ($NAME:ident, $CONST:literal, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::const_numeric!($NAME, $CONST, u32, $ENDIAN, $STRUCT_META );
    };
}

/// see [const_u16_tuple] for more details and examples.
#[macro_export]
macro_rules! const_i32_tuple {
    ($NAME:ident, $CONST:literal, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::const_numeric!($NAME, $CONST, i32, $ENDIAN, $STRUCT_META );
    };
}

/// see [const_u16_tuple] for more details and examples.
#[macro_export]
macro_rules! const_u64_tuple {
    ($NAME:ident, $CONST:literal, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::const_numeric!($NAME, $CONST, u64, $ENDIAN, $STRUCT_META );
    };
}

/// see [const_u16_tuple] for more details and examples.
#[macro_export]
macro_rules! const_i64_tuple {
    ($NAME:ident, $CONST:literal, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::const_numeric!($NAME, $CONST, i64, $ENDIAN, $STRUCT_META );
    };
}

/// This is a short hand macro for generating a new `tuple` `struct` type for numerics like u32, i32, u64, i64, f32, f64, ...
/// Typically will not be used directly but instead will be called via one of the other macros like `u16_tuple`, `i16_tuple`, ...
#[macro_export]
macro_rules! numeric_tuple {
    ($NAME:ident, $TYPE:ty, $ENDIAN:literal, $STRUCT_META:meta ) => {
        #[$STRUCT_META]
        #[byteserde(endian = $ENDIAN )]
        pub struct $NAME($TYPE);
        impl $NAME {
            pub fn value(&self) -> $TYPE {
                self.0
            }
            pub fn new(value: $TYPE) -> Self {
                $NAME(value)
            }
        }
        impl From<$TYPE> for $NAME {
            #[inline(always)]
            fn from(v: $TYPE) -> Self {
                $NAME(v)
            }
        }
        impl From<$NAME> for $TYPE {
            #[inline(always)]
            fn from(v: $NAME) -> Self {
                v.0
            }
        }
        impl std::fmt::Display for $NAME{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }
    }
}

/// Generates a `tuple` `struct` with a given name for managing a Numeric type `u16` allocated on `stack`.
///
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `ENDIAN` - endianess of the numeric type, must be either `le`, `be`, or `ne`, this will be passed directly to the `byteserde` attribute as #[byteserde(endian = "xx" )]
/// * `derive(...)` -- `must include one of` the following `ByteSerializeStack`, `ByteSerializeHeap`, or `ByteDeserializeSlice` other wise the `#[byteserde(endian = $ENDIAN)]` attribute will fail to compile.
/// Plus list of additional valid rust derive traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * `Display` - provides a human readable sting view of the `u16` value
///
/// # From
/// Note that provided implementation already includes the following `From` implementations.
/// * `From<u16>` - will take the `u16` and return tuple struct with type of `NAME` argument.
/// * `From<Name>` - will take the `struct` type from the `NAME` argument and return the `u16` value.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// use byteserde_derive::ByteSerializeStack;
/// u16_tuple!(Number, "be", derive(ByteSerializeStack, PartialEq, Debug));
///
/// let inp_num: Number = 1_u16.into(); // from u16
/// println!("inp_num: {:?}, {}", inp_num, inp_num);
/// assert_eq!(inp_num.value(), 1_u16);
///
/// let inp_num: Number = Number::new(2); // using new
/// println!("inp_num: {:?}, {}", inp_num, inp_num);
/// assert_eq!(inp_num.value(), 2_u16);
///
/// let inp_num: u16 = inp_num.into(); // to u16
/// assert_eq!(inp_num, 2_u16);
/// # }
/// ```
#[macro_export]
macro_rules! u16_tuple {
    ($NAME:ident, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::numeric_tuple!($NAME, u16, $ENDIAN, $STRUCT_META );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! i16_tuple {
    ($NAME:ident, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::numeric_tuple!($NAME, i16, $ENDIAN, $STRUCT_META );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! u32_tuple {
    ($NAME:ident, $ENDIAN:literal, $STRUCT_META:meta) => {
        $crate::numeric_tuple!($NAME, u32, $ENDIAN, $STRUCT_META );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! i32_tuple {
    ($NAME:ident, $ENDIAN:literal, $STRUCT_META:meta ) => {
        $crate::numeric_tuple!($NAME, i32, $ENDIAN, $STRUCT_META );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! u64_tuple {
    ($NAME:ident, $ENDIAN:literal, $STRUCT_META:meta ) => {
        $crate::numeric_tuple!($NAME, u64, $ENDIAN, $STRUCT_META );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! i64_tuple {
    ($NAME:ident, $ENDIAN:literal, $STRUCT_META:meta ) => {
        $crate::numeric_tuple!($NAME, i64, $ENDIAN, $STRUCT_META );
    };
}
