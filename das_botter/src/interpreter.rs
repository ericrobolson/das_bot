use std::{collections::HashMap, time::Duration};

use crate::{key::Key, timeline::Timeline, toggle::Toggle};

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
    pub fn load(&mut self, file_path: &String) {}

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

    pub fn execute(&mut self, op: Op) -> Result<(), InterpreterError> {
        // Execute main method
        match op {
            Op::Input(_) => todo!(),
        }
    }
}

pub enum InterpreterError {
    MethodNotFound(String),
}
