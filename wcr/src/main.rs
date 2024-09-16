use clap::Parser;
use std::{fs::File, io::BufReader};
use std::io::BufRead;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(
        short = 'l',
        long = "lines"
    )]
    lines: bool,
    #[arg(
        short = 'w',
        long = "words"
    )]
    words: bool,
    #[arg(
        short = 'c',
        long = "bytes"
    )]
    bytes: bool,
    #[arg(
        short = 'm',
        long = "chars"
    )]
    chars: bool
}

fn main() {
    let args = Args::parse();
    args.files.iter().for_each(|filename| {
        match File::open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut count: u32 = 0;
                loop {
                    let mut str: String = String::new();
                    let bytes = reader.read_line(&mut str);

                    match bytes {
                        Err(e) => eprintln!("Failed to read {}", e),
                        Ok(byte) => {
                            if byte == 0 {
                                break;
                            }
                            count += 1;
                        }
                    }
                }
                println!("       {} {}", count, filename);
            }
        }
    });
}
