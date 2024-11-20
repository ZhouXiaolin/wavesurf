mod differentiate;
mod integrate;
mod integration_rules;
mod integration_state;
use std::error::Error;
use std::fmt;

pub use differentiate::Differentiate;
pub use integration_state::{IntegrationState, IntegrationMethod};

#[derive(Debug)]
pub enum IntegrationError {
    MaxDepthExceeded,
    NoMethodFound,
    NotImplemented,
    InvalidInput(String),
    UnsupportedOperation(String),
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntegrationError::MaxDepthExceeded => write!(f, "Integration exceeded maximum recursion depth"),
            IntegrationError::NoMethodFound => write!(f, "No suitable integration method found"),
            IntegrationError::NotImplemented => write!(f, "This integration method is not yet implemented"),
            IntegrationError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            IntegrationError::UnsupportedOperation(msg) => write!(f, "Unsupported operation: {}", msg),
        }
    }
}

impl Error for IntegrationError {}
