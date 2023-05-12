use crate::{
    des::{ByteDeserialize, ByteDeserializer},
    error::{Result, SerDesError},
    ser::{ByteSerializeHeap, ByteSerializeStack, ByteSerializerStack},
    utils::hex::to_hex_line,
};

impl ByteSerializeStack for String {
    fn byte_serialize_stack<const CAP: usize>(
        &self,
        serializer: &mut ByteSerializerStack<CAP>,
    ) -> Result<()> {
        let len = self.len();
        if len > u32::MAX as usize {
            Err(SerDesError {
                message: format!(
                    "max string len supported is {max}, but enchountered {len}",
                    max = u32::MAX
                ),
            })
        } else {
            serializer.serialize_bytes(&(len as u32).to_be_bytes())?;
            serializer.serialize_bytes(self.as_bytes())?;
            Ok(())
        }
    }
}

impl ByteSerializeHeap for String {
    fn byte_serialize_heap(&self, ser: &mut crate::ser::ByteSerializerHeap) -> Result<()> {
        let len = self.len();
        if len > u32::MAX as usize {
            Err(SerDesError {
                message: format!(
                    "max string len supported is {max}, but enchountered {len}",
                    max = u32::MAX
                ),
            })
        } else {
            ser.serialize_bytes(&(len as u32).to_be_bytes())?;
            ser.serialize_bytes(self.as_bytes())?;
            Ok(())
        }
    }
}

impl ByteDeserialize<String> for String {
    fn byte_deserialize(deserializer: &mut ByteDeserializer) -> Result<String> {
        let len = deserializer.deserialize_be::<4, u32>()?;
        let bytes = deserializer.deserialize_bytes_slice(len as usize)?;
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err(SerDesError {
                message: format!(
                    "bytes slice is not a valid utf8 string bytes: {}",
                    to_hex_line(bytes)
                ),
            }),
        }
    }
}

impl ByteSerializeStack for char {
    fn byte_serialize_stack<const CAP: usize>(
        &self,
        serializer: &mut ByteSerializerStack<CAP>,
    ) -> Result<()> {
        let len = self.len_utf8(); // max len is 4 bytes for valid utf8
        serializer.serialize_be(len as u8)?;
        let mut bytes = [0_u8; 4];
        self.encode_utf8(&mut bytes);
        serializer.serialize_bytes(&bytes[0..len])?;
        Ok(())
    }
}

impl ByteSerializeHeap for char {
    fn byte_serialize_heap(&self, ser: &mut crate::ser::ByteSerializerHeap) -> Result<()> {
        let len = self.len_utf8(); // max len is 4 bytes for valid utf8
        ser.serialize_be(len as u8)?;
        let mut bytes = [0_u8; 4];
        self.encode_utf8(&mut bytes);
        ser.serialize_bytes(&bytes[0..len])?;
        Ok(())
    }
}
impl ByteDeserialize<char> for char {
    fn byte_deserialize(deserializer: &mut ByteDeserializer) -> Result<char> {
        let len = deserializer.deserialize_be::<1, u8>()?;
        if !(1..=4).contains(&len) {
            // if !(len >= 1 && len <= 4) {
            return Err(SerDesError {
                message: format!("max char len supported 4 but enchountered {len}"),
            });
        }

        let bytes = deserializer.deserialize_bytes_slice(len as usize)?;
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s.chars().next().unwrap()), // unwrap shoudl not panic
            Err(_) => Err(SerDesError {
                message: format!(
                    "byte slice is not a valid utf8 char. bytes: {}",
                    to_hex_line(bytes)
                ),
            }),
        }
    }
}
