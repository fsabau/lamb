pub mod term;
pub mod debruijn;
pub mod evaluate;
pub mod parser;
pub mod error;


pub fn read_file(path: &std::path::Path) -> Result<String, error::LambError> {
    let result = std::fs::read_to_string(path);
    match result {
        Ok(s) => Ok(s),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => Err(error::LambError::FileNotFound(path.to_owned())),
                _ek => panic!("{}",e),
            }
        }
    }
}

