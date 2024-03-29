use crate::{prelude::*, utils::hex::to_hex_line};

/// Default String implementation for ByteSerializeStack
///
/// # Appoach
/// * first `usize` bytes to store the length of the string
/// * remaining bytes to store the string
impl ByteSerializeStack for String {
    fn byte_serialize_stack<const CAP: usize>(&self, serializer: &mut ByteSerializerStack<CAP>) -> crate::error::Result<()> {
        let len = self.len();
        serializer.serialize_bytes_slice(&len.to_be_bytes())?;
        serializer.serialize_bytes_slice(self.as_bytes())?;
        Ok(())
    }
}

/// Default String implementation for ByteSerializeHeap
///
/// # Appoach
/// * first `usize` bytes to store the length of the string
/// * remaining bytes to store the string
impl ByteSerializeHeap for String {
    fn byte_serialize_heap(&self, ser: &mut ByteSerializerHeap) -> crate::error::Result<()> {
        let len = self.len();
        ser.serialize_bytes_slice(&len.to_be_bytes())?;
        ser.serialize_bytes_slice(self.as_bytes())?;
        Ok(())
    }
}

/// Default String implementation for ByteDeserializeSlice
///
/// # Appoach
/// * first `usize` bytes to read the length of the string
/// * remaining bytes to read the string
impl ByteDeserializeSlice<String> for String {
    fn byte_deserialize(deserializer: &mut ByteDeserializerSlice) -> crate::error::Result<String> {
        let len: usize = deserializer.deserialize_be()?;
        let bytes = deserializer.deserialize_bytes_slice(len)?;
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err(SerDesError {
                message: format!("bytes slice is not a valid utf8 string bytes: {}", to_hex_line(bytes)),
            }),
        }
    }
}

impl ByteDeserializeBytes<String> for String {
    fn byte_deserialize(des: &mut ByteDeserializerBytes) -> crate::error::Result<String> {
        let len: usize = des.deserialize_be()?;
        let bytes = des.deserialize_bytes_slice(len)?;
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err(SerDesError {
                message: format!("bytes slice is not a valid utf8 string bytes: {}", to_hex_line(bytes)),
            }),
        }
    }
}

impl ByteSerializeStack for char {
    fn byte_serialize_stack<const CAP: usize>(&self, serializer: &mut ByteSerializerStack<CAP>) -> crate::error::Result<()> {
        let len = self.len_utf8(); // max len is 4 bytes for valid utf8
        serializer.serialize_bytes_slice(&[len as u8])?;
        let mut bytes = [0_u8; 4];
        self.encode_utf8(&mut bytes);
        serializer.serialize_bytes_slice(&bytes[0..len])?;
        Ok(())
    }
}

impl ByteSerializeHeap for char {
    fn byte_serialize_heap(&self, ser: &mut crate::prelude::ByteSerializerHeap) -> crate::error::Result<()> {
        let len = self.len_utf8(); // max len is 4 bytes for valid utf8
        ser.serialize_bytes_slice(&[len as u8])?;
        let mut bytes = [0_u8; 4];
        self.encode_utf8(&mut bytes);
        ser.serialize_bytes_slice(&bytes[0..len])?;
        Ok(())
    }
}
impl ByteDeserializeSlice<char> for char {
    fn byte_deserialize(des: &mut ByteDeserializerSlice) -> crate::error::Result<char> {
        let len = des.deserialize_bytes_slice(1)?[0];
        if !(1..=4).contains(&len) {
            return Err(SerDesError {
                message: format!("max char len supported 4 but encountered {len}"),
            });
        }

        let bytes = des.deserialize_bytes_slice(len as usize)?;
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s.chars().next().unwrap()), // unwrap should not panic
            Err(_) => Err(SerDesError {
                message: format!("byte slice is not a valid utf8 char. bytes: {}", to_hex_line(bytes)),
            }),
        }
    }
}

impl ByteDeserializeBytes<char> for char {
    fn byte_deserialize(des: &mut crate::prelude::ByteDeserializerBytes) -> crate::error::Result<char> {
        let len = des.deserialize_bytes_slice(1)?[0];
        if !(1..=4).contains(&len) {
            return Err(SerDesError {
                message: format!("max char len supported 4 but encountered {len}"),
            });
        }

        let bytes = des.deserialize_bytes_slice(len as usize)?;
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s.chars().next().unwrap()), // unwrap should not panic
            Err(_) => Err(SerDesError {
                message: format!("byte slice is not a valid utf8 char. bytes: {}", to_hex_line(bytes)),
            }),
        }
    }
}
