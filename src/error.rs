use std::convert::From;

#[derive(Debug)]
pub enum LambError{
    Parse(String),
    IO(std::io::Error),
    NotDefined(String),
}

impl From<std::io::Error> for LambError {
    fn from(error: std::io::Error) -> LambError {
        LambError::IO(error)
    }
}


impl std::fmt::Display for LambError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambError::Parse(message) => write!(f, "Parsing error:\n{}", message),
            LambError::NotDefined(n) => write!(f, "{} not defined", n),
            LambError::IO(e) => write!(f,"{}",e)
        }
    }
}

impl std::error::Error for LambError { }

