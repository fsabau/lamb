use std::convert::From;
use nom::error::ParseError;

#[derive(Debug,Clone)]
pub enum LambError<'a>{
    Parse(nom::Err<(&'a str, nom::error::ErrorKind)>),
    NotDefined(String),
}

impl<'a> From<nom::Err<(&'a str, nom::error::ErrorKind)>> for LambError<'a> {
   fn from(error: nom::Err<(&'a str, nom::error::ErrorKind)>) -> LambError<'a>{
       LambError::Parse(error)
   }
}

impl<'a> std::fmt::Display for LambError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambError::Parse(e) => write!(f, "{}", e),
            LambError::NotDefined(n) => write!(f, "{} not defined", n),
        }
    }
}

impl<'a> std::error::Error for LambError<'a> { }

impl<'a> ParseError<&'a str> for LambError<'a> {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        LambError::Parse(nom::Err::Error((input, kind)))
    }
    fn append(_:&'a str, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
