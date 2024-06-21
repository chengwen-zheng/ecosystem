use std::mem::size_of;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Error: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}

fn main() -> Result<(), anyhow::Error> {
    println!("size of anyhow::Error: {}", size_of::<anyhow::Error>());

    println!("size of std::io::Error is: {}", size_of::<std::io::Error>());
    // size of parse int error
    println!(
        "size of std::num::ParseIntError is: {}",
        size_of::<std::num::ParseIntError>()
    );

    // serde json error
    println!(
        "size of serde_json::Error is: {}",
        size_of::<serde_json::Error>()
    );

    // size of String
    println!("size of String is: {}", size_of::<String>());

    // size of MyError
    println!("size of MyError is: {}", size_of::<MyError>());

    fail_with_error()?;
    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
