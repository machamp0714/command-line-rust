use clap::{Arg, App};
use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for (index, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if config.files.len() > 1 {
                    let prefix = if index == 0 { "" } else { "\n" };
                    println!("{}==> {} <==", prefix, filename);
                }

                match config.bytes {
                    Some(mut bytes) => {
                        loop {
                            let mut line = String::new();
                            let bytes_read = file.read_line(&mut line)?;
                            let line_bytes = line.bytes().take(bytes).collect::<Vec<u8>>();
                            print!("{}", String::from_utf8_lossy(&line_bytes));
                            line.clear();

                            if bytes < bytes_read || bytes_read == 0 {
                                break;
                            } else {
                                bytes -= bytes_read;
                            }
                        }
                    },
                    None => {
                        for _ in 0..config.lines {
                            let mut line = String::new();
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    }
                }
            }
        }
    }
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
                .help("Print count lines of each of the specified files.")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Print bytes of each of the specified files.")
        )
        .get_matches();

    let files  = matches.values_of_lossy("files").unwrap();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    let config = Config {
        files,
        lines: lines.unwrap(),
        bytes
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
