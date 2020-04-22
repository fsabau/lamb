use std::error::Error;
use std::path::Path;
use lamb::error::LambError;
use lamb::evaluate::{Evaluator, Strategy};
use lamb::term::Term;
use clap::App;

fn handle_opts<'a,'b>(matches: clap::ArgMatches<'a>) -> Result<Term,LambError<'b>> {
    let mut evaluator = Evaluator::new();
    let filename = matches.value_of("INPUT").unwrap();

    evaluator.eval_file(Path::new(filename)).unwrap();

    match evaluator.reduce("main", Strategy::NormalOrder) {
        Some(t) => Ok(t),
        None => Err(LambError::NotDefined("main".to_owned())),
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    
    let yaml= clap::load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("{}",handle_opts(matches)?);
    Ok(())
}
