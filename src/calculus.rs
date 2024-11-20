mod differentiate;
mod integrate;
mod integration_rules;
use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct IntegrationError(String);

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IntegrationError {}
