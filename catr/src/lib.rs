use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    // ()はユニット型
    println!("Hello, world!");
    Ok(())
}
