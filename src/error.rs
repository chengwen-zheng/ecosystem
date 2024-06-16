use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("An I/O error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("An error occurred while parsing the input: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("An error occurred while serializing or deserializing JSON: {0}")]
    Serialize(#[from] serde_json::Error),

    // #[error("An error occurred")]
    // BigError{
    //    a: String,
    //     b: Vec<String>,
    //     c: [u8; 32],
    //     d: u64,
    // },
    #[error("An error occurred: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 32],
    d: u64,
}
