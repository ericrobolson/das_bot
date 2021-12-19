mod interpreter;
mod key;
mod send_input;
mod timeline;
mod toggle;
mod tokenizer;

use interpreter::{Interpreter, InterpreterError};

#[derive(Clone, Debug)]
pub enum CliError {
    FilePathRequired,
    InterpreterError(InterpreterError),
}

fn main() -> Result<(), CliError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return Err(CliError::FilePathRequired);
    }

    let file_path = &args[1];

    let mut interpreter = Interpreter::new();
    interpreter.load(file_path.clone())?;
    interpreter.main()?;

    Ok(())
}

impl From<InterpreterError> for CliError {
    fn from(e: InterpreterError) -> Self {
        CliError::InterpreterError(e)
    }
}
