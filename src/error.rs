use std::error::Error;
use std::convert::From;

#[derive(Debug)]
pub enum EvalError<'a>{
    Parse(nom::Err<(&'a str, nom::error::ErrorKind)>),
    NotDefined(&'a str),
}

impl<'a> From<nom::Err<(&'a str, nom::error::ErrorKind)>> for EvalError<'a> {
   fn from(error: nom::Err<(&'a str, nom::error::ErrorKind)>) -> EvalError<'a>{
       EvalError::Parse(error)
   }
}

impl<'a> std::fmt::Display for EvalError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::Parse(e) => write!(f, "{}", e),
            EvalError::NotDefined(n) => write!(f, "{} not defined", n),
        }
    }
}

impl<'a> Error for EvalError<'a> { }
