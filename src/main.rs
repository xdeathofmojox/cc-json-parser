use std::env;
use std::io;
use std::process::ExitCode;

use cc_json_parser::{handle_file, json_valid, ParseOptions};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let (options, filenames) = parse_args(&args);
    let mut status = 0;

    if filenames.is_empty() {
        let stdin = io::stdin();
        let result = json_valid(&mut stdin.lock(), options);
        if result.is_err() {
            println!("Invalid: stdin - {:?}", result.err().unwrap().to_string());
            status = 1;
        } else {
            println!("Valid: stdin");
        }
    }

    for filename in filenames {
        let result = handle_file(filename, options);
        if result.is_err() {
            println!(
                "Invalid: {:?} - {:?}",
                filename,
                result.err().unwrap().to_string()
            );
            status = 1;
        } else {
            println!("Valid: {:?}", filename);
        }
    }

    ExitCode::from(status)
}

fn parse_args<'a>(args: &'a [String]) -> (ParseOptions, Vec<&'a str>) {
    let mut max_depth = None;
    let mut filenames = vec![];
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        if arg == "--max-depth" {
            max_depth = iter.next().and_then(|v| v.parse().ok());
        } else {
            filenames.push(arg.as_str());
        }
    }

    (ParseOptions { max_depth }, filenames)
}
