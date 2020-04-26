use std::path::PathBuf;

#[derive(Debug)]
pub enum LambError{
    Parse(String),
    FileNotFound(PathBuf),
    NotDefined(String),
}


impl std::fmt::Display for LambError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambError::Parse(message) => write!(f, "Parsing error:\n{}", message),
            LambError::NotDefined(n) => write!(f, "{} not defined", n),
            LambError::FileNotFound(p) => write!(f,"File not found: {}", p.to_str().unwrap())
        }
    }
}

impl std::error::Error for LambError { }

