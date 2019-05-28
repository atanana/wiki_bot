use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct NoDyk;

impl fmt::Display for NoDyk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No dyk!")
    }
}

impl Error for NoDyk {}