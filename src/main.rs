use std::error::Error;
use std::path::Path;
use lamb::error::LambError;
use lamb::evaluate::{Evaluator, Strategy};
use lamb::term::Term;
use clap::App;

fn handle_opts(matches: clap::ArgMatches) -> Result<Term,LambError> {
    let mut evaluator = Evaluator::new();
    let filename = matches.value_of("INPUT").unwrap();

    evaluator.eval_file(Path::new(filename))?;

    match evaluator.reduce("main", Strategy::NormalOrder) {
        Some(t) => Ok(t),
        None => Err(LambError::NotDefined("main".to_owned())),
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    
    let yaml= clap::load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    match handle_opts(matches) {
        Ok(t) => println!("{}", t),
        Err(e) => println!("{}",e),
    }
    Ok(())
}
