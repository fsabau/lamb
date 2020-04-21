use std::collections::HashMap;
use crate::error::LambError;
use crate::{
    term::Term,
    parser,
};



#[derive(Copy,Clone,Debug)]
pub enum Strategy {
    NormalOrder,
    ApplicativeOrder,
    CallByName,
    CallByValue,
}

pub struct Evaluator {
    pub env: HashMap<String, Term>
}


impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator { env: HashMap::new() }
    }

    pub fn add(&mut self, name: & str, term: Term) {
        self.env.insert(name.to_owned(), term); 
    }
    
    pub fn eval_file<'a>(&mut self,file: &'a str) -> Result<(), LambError<'a>> {
        let (_, v) = parser::file(file)?;
        for (name,expr) in v {
            let term = expr.to_term(&self.env)?;
            self.env.insert(name, term);
        }
        Ok(())
    }
    pub fn reduce(&self, name: &str, strategy: Strategy) -> Option<Term> {
        let t = self.env.get(name)?.clone();
        let dbt = t.to_de_bruijn().reduce(strategy);
        Some(dbt.to_term())
    }
}
