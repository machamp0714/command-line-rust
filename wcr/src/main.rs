use clap::Parser;

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
    println!("{:?}", args);
}
