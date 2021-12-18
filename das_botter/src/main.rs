mod interpreter;
mod key;
mod send_input;
mod timeline;
mod toggle;

use interpreter::Interpreter;
use key::*;
use rand::Rng;
use std::env;
use std::time::Duration;
use timeline::*;
use toggle::Toggle;

const MAX_TIME_DELTA: u64 = 42069 * 6;

// TODO: add 'random' method that can generate a random duration
// TODO: add a method that can sleep for a given duration

#[derive(Clone, Copy, Debug)]
pub enum CliError {
    FilePathRequired,
}

fn main() -> Result<(), CliError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return Err(CliError::FilePathRequired);
    }

    let file_path = &args[1];

    let mut interpreter = Interpreter::new();
    interpreter.load(file_path);

    println!("file: {:?}", file_path);

    Ok(())
}
