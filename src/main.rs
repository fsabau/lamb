use std::error::Error;
use lambda::term::Term;
use lambda::parser;

fn main() -> Result<(),Box<dyn Error>> {
    let term = Term::App(
        Box::new(Term::Abs(
            'x',
            Box::new(Term::App(
                Box::new(Term::Var(2)),
                Box::new(Term::Var(0)),
            )),
        )),
        Box::new(Term::Var(3)),
    );

    let (_,mut expr) = parser::term(r"(\x.\y.x) t f")?; 
    
    println!("{}", expr);
    expr.to_de_bruijn();
    let mut expr = expr.normal_order();
    expr.to_chars();
    println!("{}", expr);
    Ok(())
}
