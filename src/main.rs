use std::error::Error;
use lambda::term::{Term, evaluate::{Evaluator, Strategy} };
use lambda::parser;



fn main() -> Result<(),Box<dyn Error>> {
    // let term = Term::App(
    //     Box::new(Term::Abs(
    //         'x',
    //         Box::new(Term::App(
    //             Box::new(Term::Var(2)),
    //             Box::new(Term::Var(0)),
    //         )),
    //     )),
    //     Box::new(Term::Var(3)),
    // );
    
    let file = std::fs::read_to_string("test.lamb")?;
    let mut eval = Evaluator::new();

    eval.eval_file(&file).unwrap();
    println!("{}", eval.reduce("main", Strategy::NormalOrder).unwrap());
    Ok(())
}
