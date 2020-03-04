use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct CacheError {
    details: String,
}

impl CacheError {
    pub fn new(msg: &str) -> CacheError {
        CacheError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CacheError {
    fn description(&self) -> &str {
        &self.details
    }
}
