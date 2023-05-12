

#[derive(Debug)]
pub struct SerDesError {
    pub message: String,
}

pub type Result<T> = std::result::Result<T, crate::error::SerDesError>;
pub type Result2<T> = std::result::Result<T, std::io::Error>;
