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

impl std::str::FromStr for Strategy {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Strategy,Self::Err> {
        match s {
            "normal" | "n" => Ok(Strategy::NormalOrder),
            "applicative" | "a" => Ok(Strategy::ApplicativeOrder),
            "call_by_name" | "cbn" => Ok(Strategy::CallByName),
            "call_by_value" | "cbv" => Ok(Strategy::CallByValue),
            _ => Err("No match for Strategy"),
        }
    }
}

pub struct Evaluator {
    pub env: HashMap<String, Term>,
}


impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator { env: HashMap::new() }
    }

    pub fn add(&mut self, name: & str, term: Term) {
        self.env.insert(name.to_owned(), term); 
    }


    pub fn eval_repl(&mut self, line: &str, strategy: Strategy) -> Result<(),LambError> {
        let (_, stmt) = parser::parse(parser::repl, line)?;
        match stmt {
            Statement::Import(path) => self.eval_file(&path)?,
            Statement::Let(name, expr) => self.add(&name, expr.to_term(&self.env)?),
            Statement::Expr(e) => println!("{}", e.to_term(&self.env)?.reduce(strategy))
        }
        Ok(())
    }
    
    pub fn eval_file(&mut self, path: &Path) -> Result<(), LambError> {
        let contents: String = crate::read_file(path)?;
        let (_, v) = parser::parse(parser::file, &contents)?;
        for stmt in v {
            match stmt {
               Statement::Import(path) => self.eval_file(&path)?,
               Statement::Let(name,expr) => self.add(&name, expr.to_term(&self.env)?),
               Statement::Expr(_) => (),
            }
        }
        Ok(())
    }
    
    pub fn get(&self, name: &str) -> Result<Term, LambError> {
        match self.env.get(name) {
            Some(t) => Ok(t.clone()),
            None => Err(LambError::NotDefined(name.to_owned())) 
        }
    }

}
