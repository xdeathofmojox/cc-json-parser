use std::env;
use std::io;
use std::process::ExitCode;

use cc_json_parser::{handle_file, json_valid};

fn main() -> ExitCode {
    let filenames: Vec<String> = env::args().skip(1).collect();
    let mut status = 0;

    if filenames.is_empty() {
        let stdin = io::stdin();
        let result = json_valid(&mut stdin.lock());
        if result.is_err() {
            println!("Invalid: stdin - {:?}", result.err().unwrap().to_string());
            status = 1;
        } else {
            println!("Valid: stdin");
        }
    }

    for filename in filenames {
        let result = handle_file(filename.as_str());
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
