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
/// let inp_pwd: Password = b"12345".as_slice().into(); // from slice
/// println!("inp_pwd: {:?}, {}", inp_pwd, inp_pwd);
/// assert_eq!(inp_pwd.as_bytes(), b"12345-----");
///
/// // Align=Right / Len=10 / Padding=Space
/// string_ascii_fixed!(Username, 10, b' ', true, PartialEq);
/// let inp_usr1: Username = b"12345".as_slice().into(); // from slice
/// let inp_usr2: Username = b"     12345".into(); // from array of matching len
/// println!("inp_usr1: {:?}, {}", inp_usr1, inp_usr1);
/// println!("inp_usr2: {:?}, {}", inp_usr2, inp_usr2);
/// assert_eq!(inp_usr1.as_bytes(), b"     12345");
/// assert_eq!(inp_usr2.as_bytes(), b"     12345");
/// assert_eq!(inp_usr1, inp_usr2);
/// # }
/// ```
#[macro_export]
macro_rules! string_ascii_fixed {
    ($NAME:ident, $LEN:literal, $PADDING:literal, $RIGHT_ALIGN: literal, $($DERIVE:ty),* ) => {
        #[derive( $($DERIVE),* ) ]
        pub struct $NAME([u8; $LEN]);
        impl $NAME{
            pub fn as_bytes(&self) -> &[u8; $LEN] { 
                &self.0
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
/// assert_eq!(inp_char.as_byte(), b'1');
/// 
/// let inp_char: Char = [b'1'].into(); // from array
/// println!("inp_char: {:?}, {}", inp_char, inp_char);
/// assert_eq!(inp_char.as_byte(), b'1');
/// # }
/// ```
#[macro_export]
macro_rules! char_ascii {
    ($NAME:ident, $($derive:ty),*) => {
        /// Tuple struct with a `u8` buffer to represent an ascii char.
        #[derive( $($derive),* )]
        pub struct $NAME(u8);
        impl $NAME {
            /// proves access to the `u8` byte
            pub fn as_byte(&self) -> u8 {
                self.0
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

#[macro_export]
macro_rules! u32_tuple {
    ($NAME:ident, $ENDIAN:literal, $($derive:ty),*) => {
        /// Tuple struct with a `u32` value
        #[derive( $($derive),* )]
        #[byteserde(endian = $ENDIAN )]
        pub struct $NAME(u32);
        impl $NAME {
            pub fn value(&self) -> u32 {
                self.0
            }
        }
        impl From<u32> for $NAME {
            fn from(v: u32) -> Self {
                $NAME(v)
            }
        }
        impl std::fmt::Display for $NAME{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }
    }
}

#[macro_export]
macro_rules! u64_tuple {
    ($NAME:ident, $ENDIAN:literal, $($derive:ty),*) => {
        /// Tuple struct with a `u64` value
        #[derive( $($derive),* )]
        #[byteserde(endian = $ENDIAN )]
        pub struct $NAME(u64);
        impl $NAME {
            pub fn value(&self) -> u64 {
                self.0
            }
        }
        impl From<u64> for $NAME {
            fn from(v: u64) -> Self {
                $NAME(v)
            }
        }
        impl std::fmt::Display for $NAME{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }
    }
}