use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .author("machamp0714")
        .version("1.0.0")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .takes_value(false)
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let bytes = matches.is_present("bytes");
    let lines = matches.is_present("lines");
    let chars = matches.is_present("chars");
    let words = matches.is_present("words");

    Ok(Config {
        files,
        bytes,
        lines,
        chars,
        words,
    })
}
