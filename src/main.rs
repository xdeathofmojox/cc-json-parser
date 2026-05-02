use std::io;
use std::process::ExitCode;

use clap::Parser;

use cc_json_parser::{handle_file, json_valid, ParseOptions};

#[derive(Parser)]
#[command(name = "cc-json-parser", about = "Validate JSON files or stdin")]
struct Cli {
    /// Files to validate (reads from stdin if none provided)
    files: Vec<String>,

    /// Maximum nesting depth (unlimited if not set)
    #[arg(long)]
    max_depth: Option<usize>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let options = ParseOptions { max_depth: cli.max_depth };
    let mut status = 0;

    if cli.files.is_empty() {
        let stdin = io::stdin();
        let result = json_valid(&mut stdin.lock(), options);
        if result.is_err() {
            println!("Invalid: stdin - {:?}", result.err().unwrap().to_string());
            status = 1;
        } else {
            println!("Valid: stdin");
        }
    }

    for filename in &cli.files {
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
