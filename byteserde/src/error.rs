use std::fmt::{self, Display};

pub struct SerDesError {
    pub message: String,
}

impl std::error::Error for SerDesError {} 

impl fmt::Debug for SerDesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SerDesError")
            .field("message", &self.message)
            .finish()
        // f.write_str(format!("SerDesError {{\n\tmessage: {}\n}}", self.message).as_str())
    }
}
impl Display for SerDesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SerDesError")
            .field("message", &self.message)
            .finish()
    }
}

pub type Result<T> = std::result::Result<T, crate::error::SerDesError>;
