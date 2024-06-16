use ecosystem::MyError;

fn main() -> Result<(), anyhow::Error> {
    println!(
        "size of anyhow::Error: {}",
        std::mem::size_of::<anyhow::Error>()
    );

    println!(
        "size of std::io::Error is: {}",
        std::mem::size_of::<std::io::Error>()
    );
    // size of parse int error
    println!(
        "size of std::num::ParseIntError is: {}",
        std::mem::size_of::<std::num::ParseIntError>()
    );

    // serde json error
    println!(
        "size of serde_json::Error is: {}",
        std::mem::size_of::<serde_json::Error>()
    );

    // size of String
    println!("size of String is: {}", std::mem::size_of::<String>());

    // size of MyError
    println!("size of MyError is: {}", std::mem::size_of::<MyError>());

    fail_with_error()?;
    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
