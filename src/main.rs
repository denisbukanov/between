use clap::Parser;
use parse_between::{extract_data_extended, Config};
use std::fs::File;
use std::io;
use std::io::prelude::Read;
use std::path::Path;

/// Small utility which extracts data between some prefix and suffix
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Read data from standard input
    #[clap(short, long)]
    stdin: bool,

    /// Prefix to data
    #[clap(long)]
    prefix: String,

    /// Suffix to data
    #[clap(long)]
    suffix: String,

    /// Keep prefix in the output (default: no)
    #[clap(long)]
    keep_prefix: bool,

    /// Keep prefix in the output (default: no)
    #[clap(long)]
    keep_suffix: bool,

    /// Path to file to read data from
    input: Option<String>,
}

fn read(f: &mut dyn Read) -> String {
    let mut src = String::new();
    let msg = "Error while reading".to_string();
    f.read_to_string(&mut src)
        .unwrap_or_else(|_| panic!("{}", msg));
    src.trim().to_string()
}

fn read_file(path: &str) -> String {
    if !Path::new(path).exists() {
        panic!("File {} does not exist", path);
    }
    let msg = format!("Error while reading file '{}'", path);
    let mut f = File::open(path).unwrap_or_else(|_| panic!("{}", msg));
    read(&mut f)
}

fn main() {
    let args = Args::parse();
    let source = match args.input {
        None => read(&mut io::stdin()),
        Some(src) => read_file(&src),
    };
    if source.is_empty() {
        return;
    }
    let config = Config {
        prefix: args.prefix,
        suffix: args.suffix,
        keep_prefix: args.keep_prefix,
        keep_suffix: args.keep_suffix,
        trim: false,
    };
    for piece in extract_data_extended(&source, config) {
        println!(">>>");
        println!("{piece}");
        println!("<<<");
    }
}
