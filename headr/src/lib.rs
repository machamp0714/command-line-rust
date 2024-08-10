use clap::{Arg, App};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    count: String,
    bytes: String,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("1.0.0")
        .author("machamp0714")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .default_value("10")
                .conflicts_with("bytes")
                .help("Print count lines of each of the specified files.")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Print bytes of each of the specified files.")
        )
        .get_matches();

    let files  = matches.values_of_lossy("files").unwrap();
    let count = matches.value_of("count").unwrap();
    let bytes = matches.value_of("bytes").unwrap();

    let config = Config {
        files,
        count: count.to_string(),
        bytes: bytes.to_string()
    };

    Ok(config)
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)), // Err(val.into()) or Err(Into::into(val))
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string())
}
