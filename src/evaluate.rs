use std::collections::HashMap;
use std::path::Path;
use crate::error::LambError;
use crate::parser::ast::Statement;
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
    
    pub fn eval_file<'a>(&mut self, path: &Path) -> Result<(), LambError<'a>> {
        let file = std::fs::read_to_string(path)?;
        let (_, v) = parser::file(&file).unwrap();
        for stmt in v {
            match stmt {
               Statement::Import(path) => self.eval_file(&path)?,
               Statement::Let(name,expr) => self.add(&name,expr.to_term(&self.env)?),
               Statement::Expr(_) => (),
            }
        }
        Ok(())
    }
    pub fn reduce(&self, name: &str, strategy: Strategy) -> Option<Term> {
        let t = self.env.get(name)?.clone();
        let dbt = t.to_de_bruijn().reduce(strategy);
        Some(dbt.to_term())
    }
}
