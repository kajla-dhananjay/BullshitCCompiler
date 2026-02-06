use std::env;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let status = Command::new("gcc")
        .args(&args)
        .status();

    match status {
        Ok(status) => {
            match status.code() {
                Some(code) => exit(code),
                None => exit(1),
            }
        }
        Err(err) => {
            eprintln!("ccc: failed to execute: {}", err);
            exit(1);
        }
    }
}

