use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                for line_result in file.lines() {
                    let line = line_result?;
                    println!("{}", line);
                }
            }
        }
    }
    Ok(()) // ()はユニット型
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("1.0")
        .author("machamp0714")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number the output lines, starting at 1")
                .conflicts_with("number_nonblank")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number the output lines, starting at 1, omitting blank lines")
                .takes_value(false),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number");
    let number_nonblank_lines = matches.is_present("number_nonblank");

    let config = Config {
        files,
        number_lines,
        number_nonblank_lines,
    };

    Ok(config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
