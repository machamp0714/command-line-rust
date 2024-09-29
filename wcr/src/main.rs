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

    let mut total_line_count: usize = 0;
    let mut total_word_count: usize = 0;
    let mut total_byte_count: usize = 0;

    args.files.iter().for_each(|filename| {
        match File::open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut reader: BufReader<File> = BufReader::new(file);
                let mut line_count: usize = 0;
                let mut word_count: usize = 0;
                let mut byte_count: usize = 0;

                loop {
                    let mut str: String = String::new();
                    let bytes = reader.read_line(&mut str);

                    match bytes {
                        Err(e) => eprintln!("Failed to read {}", e),
                        Ok(byte) => {
                            if byte == 0 {
                                break;
                            }
                            let iter = str.split_whitespace();
                            word_count += iter.count();
                            line_count += 1;
                            byte_count += byte;
                        }
                    }
                }

                if args.files.len() > 1 {
                    total_line_count += line_count;
                    total_word_count += word_count;
                    total_byte_count += byte_count;
                }

                output_result(&args, line_count, word_count, byte_count);
                println!(" {}", filename);
            }
        }
    });

    if args.files.len() > 1 {
        output_result(&args, total_line_count, total_word_count, total_byte_count);
        println!(" total");
    }
}

fn output_result(args: &Args, line_count: usize, word_count: usize, byte_count: usize) {
    if !args.lines && !args.words && !args.bytes {
        print!(
            "{:>align$}{:>align$}{:>align$}",
            line_count,
            word_count,
            byte_count,
            align = 8
        )
    } else {
        if args.lines {
            print!("{:>align$}", line_count, align = 8);
        }
        if args.words {
            print!("{:>align$}", word_count, align = 8);
        }
        if args.bytes {
            print!("{:>align$}", byte_count, align = 8);
        }
    }
}
