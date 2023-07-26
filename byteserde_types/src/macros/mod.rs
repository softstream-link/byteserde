/// Generates a `tuple` `struct` with a given name for managing an ascii string of fixed length. Buffer is `stack` allocated using `[u8; LEN]`.
/// Macro argument signature is as follows:
///
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `LEN` - length of the buffer that will contain ascii string in bytes
/// * `PADDING` - padding byte to be used when creating string
/// * `RIGHT_ALIGN` - boolean flag to indicate if string should be right aligned or left aligned
/// * `[derive, ...]` -- list of traits to derive for the struct, must be valid rust traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * `Debug` & `Display` - provides a human readable sting view of the `[u8; LEN]` byte buffer as a utf-8 string
///
/// # From
/// Note that provided implementation already includes the following `From` implementations.
/// * `From<&[u8]>` - will take upto `LEN` bytes from the slice and pad if necessary usig `PADDING` argument.
/// * `From<&[u8; LEN]>` - must match the length of the buffer, will not pad.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// // Align=Left / Len=10 / Padding=Minus
/// string_ascii_fixed!(Password, 10, b'-', false,); // NOTE required comma after alignment when you dont provide a single derive argument
/// let inp_pwd = Password::new(*b"1234567890");
///
/// let inp_pwd: Password = b"12345".as_slice().into(); // from slice
/// println!("inp_pwd: {:?}, {}", inp_pwd, inp_pwd);
/// assert_eq!(inp_pwd.value(), b"12345-----");
///
/// // Align=Right / Len=10 / Padding=Space
/// string_ascii_fixed!(Username, 10, b' ', true, PartialEq);
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
    ($NAME:ident, $LEN:literal, $PADDING:literal, $RIGHT_ALIGN: literal, $($DERIVE:ty),* ) => {
        #[derive( $($DERIVE),* ) ]
        pub struct $NAME([u8; $LEN]);
        impl $NAME{
            pub fn value(&self) -> &[u8; $LEN] {
                &self.0
            }
            pub fn new(value: [u8; $LEN]) -> Self {
                $NAME(value)
            }
        }
        impl From<&[u8]> for $NAME {
            ///  Runt time check for capacity, Takes defensively and upto `LEN`, never overflows.
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
                f.debug_tuple( stringify! ($NAME) )
                .field(&String::from_utf8_lossy(&self.0))
                .finish()
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
/// * `[derive, ...]` -- list of traits to derive for the struct, must be valid rust traits
///
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * `Debug` & `Display` - provides a human readable sting view of the `u8` byte as utf-8 char
///
/// # From
/// Note that provided implementation already includes the following `From` implementations.
/// * `From<u8>` - will take the `u8` byte and return tupe struct with type of `NAME` agrument.
/// * `From<[u8; 1]>` - will take the first byte of the array and return tupe struct with type of `NAME` agrument.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// char_ascii!(Char, PartialEq);
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
    ($NAME:ident, $($DERIVE:ty),*) => {
        /// Tuple struct with a `u8` buffer to represent an ascii char.
        #[derive( $($DERIVE),* )]
        pub struct $NAME(u8);
        impl $NAME {
            /// proves access to the `u8` byte
            pub fn value(&self) -> u8 {
                self.0
            }
            pub fn new(value: u8) -> Self {
                $NAME(value)
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
        // utf8 `char` based impls
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple( stringify! ($NAME) )
                    .field(&char::from_u32(self.0 as u32).ok_or(std::fmt::Error)?)
                    .finish()
            }
        }
        /// utf8 `char` based impls
        impl std::fmt::Display for $NAME{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &char::from_u32(self.0 as u32).ok_or(std::fmt::Error)?)
            }
        }
    }
}

/// Generates a `tuple` `struct` with a given name and a private ascii char allocated on `stack` using `u8` whose
/// value always set to parameter `CONST`.
/// 
/// # Arguments
/// * `NAME` - name of the struct to be generated
/// * `CONST` - `u8` byte value to be used as the value behind this struct
/// * `[derive, ...]` -- list of traits to derive for the struct, must be valid rust traits
/// 
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * `Debug` & `Display` - provides a human readable sting view of the `u8` byte as utf-8 char
/// * `ByteDeserializeSlice`- provides an implementation for deserializing from a byte stream, which `will panic` if value on the 
/// stream does `not` match the `CONST` value.
/// 
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// use byteserde::prelude::*;
/// const_char_ascii!(One, b'1', PartialEq);
/// let inp_const = One::default();
/// println!("inp_const: {:?}, {}", inp_const, inp_const);
/// assert_eq!(inp_const.value(), b'1');
/// # }
/// ```
#[macro_export]
macro_rules! const_char_ascii {
    ($NAME:ident, $CONST:literal, $($DERIVE:ty),*) => {
        #[derive( $($DERIVE),* )]
        pub struct $NAME(u8);
        impl $NAME {
            pub fn to_char() -> char {
                char::from_u32($CONST as u32).ok_or(std::fmt::Error).unwrap()
            }
            pub fn value(&self) -> u8 {
                self.0
            }
            pub fn as_slice() -> &'static [u8]{
                &[$CONST]
            }
        }
        impl Default for $NAME {
            fn default() -> Self {
                $NAME($CONST)
            }
        }
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple( stringify! ($NAME) )
                    .field(&char::from_u32(u32::from( $CONST )).ok_or(std::fmt::Error)?)
                    .finish()
            }
        }
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    &char::from_u32(u32::from( $CONST )).ok_or(std::fmt::Error)?
                )
            }
        }
        impl ::byteserde::des_slice::ByteDeserializeSlice<$NAME> for $NAME {
            #[allow(clippy::just_underscores_and_digits)]
            fn byte_deserialize(des: &mut ::byteserde::prelude::ByteDeserializerSlice) -> ::byteserde::prelude::Result<$NAME> {
                let _0 = des.deserialize_u8()?;
                match _0 == $CONST {
                    true => Ok($NAME::default()),
                    false => {
                        let ty = $NAME::default();
        
                        Err(SerDesError {
                            message: format!(
                                "Type {:?} expected: 0x{:02x} actual: 0x{:02x}",
                                ty, $CONST, _0
                            ),
                        })
                    }
                }
            }
        }
        impl ::byteserde::des_bytes::ByteDeserializeBytes<$NAME> for $NAME {
            #[allow(clippy::just_underscores_and_digits)]
            fn byte_deserialize(des: &mut ::byteserde::prelude::ByteDeserializerBytes) -> ::byteserde::prelude::Result<$NAME> {
                let _0 = des.deserialize_u8()?;
                match _0 == $CONST {
                    true => Ok($NAME::default()),
                    false => {
                        let ty = $NAME::default();
        
                        Err(SerDesError {
                            message: format!(
                                "Type {:?} expected: 0x{:02x} actual: 0x{:02x}",
                                ty, $CONST, _0
                            ),
                        })
                    }
                }
            }
        }
    };
}

/// This is a short hand macro for generateing a new `tuple` `struct` type for numerics like u32, i32, u64, i64, f32, f64, ...
/// Typically will not be used directly but instead will be called via one of the other macros like `u16_tuple`, `i16_tuple`, ...
#[macro_export]
macro_rules! numeric_tuple {
    ($NAME:ident, $TYPE:ty, $ENDIAN:literal, $($DERIVE:ty),* ) => {
        // #[derive( $($derive),* )]
        #[derive( $($DERIVE),* )]
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
            fn from(v: $TYPE) -> Self {
                $NAME(v)
            }
        }
        impl From<$NAME> for $TYPE {
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
/// * `[derive, ...]` -- `must include one of` the following `ByteSerializeStack`, `ByteSerializeHeap`, or `ByteDeserializeSlice` other wise the `#[byteserde(endian = $ENDIAN)]` attribute will fail to compile. 
/// Plus list of additional valid rust derive traits 
/// 
/// # Derives
/// Note that provided implementation already includes several traits which `SHOULD NOT` be included in the derive list.
/// * `Display` - provides a human readable sting view of the `u16` value
/// 
/// # From
/// Note that provided implementation already includes the following `From` implementations.
/// * `From<u16>` - will take the `u16` and return tupe struct with type of `NAME` agrument.
/// * `From<Name>` - will take the `struct` type from the `NAME` argument and return the `u16` value.
/// 
/// # Examples
/// ```
/// # #[macro_use] extern crate byteserde_types; fn main() {
/// u16_tuple!(Number, "be", byteserde_derive::ByteSerializeStack, PartialEq, Debug);
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
    ($NAME:ident, $ENDIAN:literal, $($DERIVE:ty),*) => {
        $crate::numeric_tuple!($NAME, u16, $ENDIAN, $($DERIVE),* );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! i16_tuple {
    ($NAME:ident, $ENDIAN:literal, $($DERIVE:ty),*) => {
        $crate::numeric_tuple!($NAME, i16, $ENDIAN, $($DERIVE),* );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! u32_tuple {
    ($NAME:ident, $ENDIAN:literal, $($DERIVE:ty),*) => {
        $crate::numeric_tuple!($NAME, u32, $ENDIAN, $($DERIVE),* );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! i32_tuple {
    ($NAME:ident, $ENDIAN:literal, $($DERIVE:ty),* ) => {
        $crate::numeric_tuple!($NAME, i32, $ENDIAN, $($DERIVE),* );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! u64_tuple {
    ($NAME:ident, $ENDIAN:literal, $($DERIVE:ty),* ) => {
        $crate::numeric_tuple!($NAME, u64, $ENDIAN, $($DERIVE),* );
    };
}

/// see [u16_tuple] for more details and examples.
#[macro_export]
macro_rules! i64_tuple {
    ($NAME:ident, $ENDIAN:literal, $($DERIVE:ty),* ) => {
        $crate::numeric_tuple!($NAME, i64, $ENDIAN, $($DERIVE),* );
    };
}
