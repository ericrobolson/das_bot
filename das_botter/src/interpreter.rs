use crate::{
    key::Key,
    timeline::Timeline,
    toggle::Toggle,
    tokenizer::{self, Token, TokenizerErr},
};
use std::{collections::HashMap, time::Duration};

const FILE_EXTENSION: &'static str = ".bot.lisp";

#[derive(Clone, Debug, PartialEq)]
pub enum InterpreterError {
    MethodNotFound(String),
    FileNotFound {
        file_path: String,
    },
    InvalidFile {
        file_path: String,
        expected_extension: String,
    },
    TokenizerErr(TokenizerErr),
}
impl From<TokenizerErr> for InterpreterError {
    fn from(e: TokenizerErr) -> Self {
        InterpreterError::TokenizerErr(e)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Input(Input),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Input {
    pub duration: Duration,
    pub key: Key,
    pub toggle: Toggle,
}

pub struct Interpreter {
    timeline: Timeline,
    environment: HashMap<String, Vec<Op>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = HashMap::new();

        Self {
            timeline: Timeline::new(),
            environment,
        }
    }

    pub fn reset(&mut self) {
        self.timeline.clear();
    }

    pub fn load(&mut self, file_path: String) -> Result<(), InterpreterError> {
        if !file_path.ends_with(FILE_EXTENSION) {
            return Err(InterpreterError::InvalidFile {
                file_path,
                expected_extension: FILE_EXTENSION.to_string(),
            });
        }

        match std::fs::read_to_string(&file_path) {
            Ok(contents) => {
                //
                let tokens = tokenizer::execute(&contents)?;

                for token in tokens {
                    println!("{:#?}", token);
                }
                Ok(())
            }
            Err(_) => Err(InterpreterError::FileNotFound { file_path }),
        }
    }

    pub fn execute_method(&mut self, method: &str) -> Result<(), InterpreterError> {
        match self.environment.get(method) {
            Some(method) => {
                for op in method.clone() {
                    self.execute(op)?;
                }
                Ok(())
            }
            None => Err(InterpreterError::MethodNotFound(method.to_string())),
        }
    }

    /// Executes the main method
    pub fn main(&mut self) -> Result<(), InterpreterError> {
        self.execute_method("main")
    }

    pub fn execute(&mut self, op: Op) -> Result<(), InterpreterError> {
        // Execute main method
        match op {
            Op::Input(_) => todo!(),
        }
    }
}
