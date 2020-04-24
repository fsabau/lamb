use std::path::PathBuf;
use std::collections::HashMap;
use crate::term::Term;
use crate::error::LambError;

#[derive(Debug,Clone)]
pub enum Expr {
    Var(char),
    Ident(String),
    Abs(char, Box<Expr>),
    App(Box<Expr>,Box<Expr>),
}

#[derive(Debug,Clone)]
pub enum Statement {
    Import(PathBuf),
    Let(String, Expr),
    Expr(Expr)
}


impl Expr {
    pub fn to_term(self, env: &HashMap<String, Term>) -> Result<Term,LambError>  {
          Ok(match self {
              Expr::Var(x) => Term::Var(x),
              Expr::Abs(x, e) => Term::Abs(x, Box::new(e.to_term(env)?)),
              Expr::App(e1,e2) => Term::App(Box::new(e1.to_term(env)?), Box::new(e2.to_term(env)?)),
              Expr::Ident(name) => match (*env).get(&name) {
                  None => return Err(LambError::NotDefined(name)),
                  Some(t) => (t).clone(),
              }
          })
    }
}

