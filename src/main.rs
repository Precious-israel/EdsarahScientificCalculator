//! # Edsarah Software and Solution — Scientific Calculator (CLI)
//!
//! Command line front end for the `edsarah_calculator` library.

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

use edsarah_calculator::{evaluate, format_result, HELP_TEXT};

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.is_empty() {
        return run_interactive_prompt();
    }

    let command = arguments[0].trim().to_ascii_lowercase();

    if matches!(command.as_str(), "help" | "-h" | "--help" | "?") {
        println!("{HELP_TEXT}");
        return ExitCode::SUCCESS;
    }

    if matches!(command.as_str(), "-v" | "--version") {
        println!("edsarah-calculator {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    run_once(&arguments[0], &arguments[1..])
}

fn run_once(operation: &str, operands: &[String]) -> ExitCode {
    match evaluate(operation, operands) {
        Ok(value) => {
            println!("{}", format_result(value));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_interactive_prompt() -> ExitCode {
    println!("Edsarah Software and Solution — Scientific Calculator");
    println!("Type 'help' for the list of operations, or 'quit' to exit.\n");

    let stdin = io::stdin();

    loop {
        print!("calc> ");
        if io::stdout().flush().is_err() {
            eprintln!("Error: could not write to standard output");
            return ExitCode::FAILURE;
        }

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => {
                println!("\nGoodbye!");
                return ExitCode::SUCCESS;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error: could not read input ({error})");
                return ExitCode::FAILURE;
            }
        }

        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        let operation = parts.next().unwrap_or_default().to_ascii_lowercase();
        let operands: Vec<String> = parts.map(|part| part.to_string()).collect();

        match operation.as_str() {
            "quit" | "exit" | "q" => {
                println!("Goodbye!");
                return ExitCode::SUCCESS;
            }
            "help" | "?" => {
                println!("{HELP_TEXT}");
                continue;
            }
            _ => {}
        }

        match evaluate(&operation, &operands) {
            Ok(value) => println!("= {}", format_result(value)),
            Err(error) => println!("Error: {error}"),
        }
    }
}