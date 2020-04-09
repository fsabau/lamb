use std::collections::HashMap;
use crate::error::EvalError;
use crate::{
    term::Term,
    parser,
};

pub struct Evaluator<'a> {
    pub env: HashMap< &'a str, Term>
}


impl<'a> Evaluator<'a> {
    pub fn new() -> Evaluator<'a> {
        Evaluator { env: HashMap::new() }
    }

    pub fn add(&mut self, name: &'a str, term: Term) {
        self.env.insert(name, term); 
    }
    
    pub fn eval<E>(expr: &'a str) -> Result<Term, EvalError<'a>> {
        let (_,mut t) = parser::term(expr)?;
        t.to_de_bruijn();
        Ok(t.normal_order())
    }
}
