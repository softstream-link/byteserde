use std::fmt;

pub struct SerDesError {
    pub message: String,
}

impl fmt::Debug for SerDesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("SerDesError {{\n\tmessage: {}\n}}", self.message).as_str())
    }
}

pub type Result<T> = std::result::Result<T, crate::error::SerDesError>;
