use std::error::Error;
use std::path::Path;
use std::io::{BufRead, Write};
use lamb::error::LambError;
use lamb::evaluate::{Evaluator, Strategy};
use lamb::term::Term;
use clap::value_t;
use clap::App;

#[derive(Debug,Copy,Clone)]
struct Opts {
    pub strategy: Strategy,
    pub verbosity: u64,
}


fn repl(options: &mut Opts) -> Result<(),LambError> {
    let mut evaluator = Evaluator::new();
    loop {
        print!("λ: ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();
        match line.trim_end() {
            "exit" | "quit" => { break; },
            s =>  evaluator.eval_repl(s,options.strategy)?,
        }
    }
    Ok(())
}


fn file(path: &Path, options: &Opts) -> Result<(), LambError> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_file(path)?;
    evaluator.get("main")?.reduce(options.strategy);
    Ok(())
}

fn handle_opts(matches: clap::ArgMatches) -> Result<(),LambError> {
    
    let verbosity = matches.occurrences_of("verbose");
    let strategy: Strategy = value_t!(matches,"strategy", Strategy).unwrap_or(Strategy::NormalOrder);
   
    let mut opts = Opts { strategy, verbosity };
    let input = matches.value_of("INPUT");
    
    match input {
        Some(p) => file(Path::new(p), &opts),
        None => repl(&mut opts),
     }
}

fn main() -> Result<(),Box<dyn Error>> {
    
    let yaml= clap::load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    match handle_opts(matches) {
        Ok(_) => (),
        Err(e) => println!("{}",e),
    }
    Ok(())
}
