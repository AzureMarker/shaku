//! Holds the overall parsing context logging e.g. parsing errors

use std::cell::RefCell;
use std::fmt::Display;

#[derive(Default)]
pub struct ParsingContext {
    errors: RefCell<Option<Vec<String>>>,
    warnings: RefCell<Option<Vec<String>>>,
}

impl ParsingContext {
    pub fn new() -> Self {
        ParsingContext { 
            errors: RefCell::new(Some(Vec::new())),
            warnings: RefCell::new(Some(Vec::new())),
        }
    }

    /// Add a new error message for this parsing context
    pub fn error<T: Display>(&self, msg: T) {
        self.errors
            .borrow_mut()
            .as_mut()
            .unwrap()
            .push(msg.to_string());
    }

    /// Add a new warning message for this parsing context
    pub fn warn<T: Display>(&self, msg: T) {
        self.warnings
            .borrow_mut()
            .as_mut()
            .unwrap()
            .push(msg.to_string());
    }

    /// Check if any errors were added and display warnings if any.
    /// If so, return them wrapped as an `Result::Err`
    pub fn check(self) -> Result<(), String> {
        // Display warnings if any
        let warnings = self.warnings.borrow_mut().take().unwrap();
        if !warnings.is_empty() {
            println!("{} warnings:", warnings.len());
            for warn in warnings {
                println!("\t# {}", &warn);
            }
        }

        // Handle errors
        let mut errors = self.errors.borrow_mut().take().unwrap();
        match errors.len() {
            0 => Ok(()),
            1 => Err(errors.pop().unwrap()),
            n => {
                let mut msg = format!("{} errors:", n);
                for err in errors {
                    msg.push_str("\n\t# ");
                    msg.push_str(&err);
                }
                Err(msg)
            }
        }
    }
}

impl Drop for ParsingContext {
    fn drop(&mut self) {
        if self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}
