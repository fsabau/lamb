pub mod debruijn;
pub mod evaluate;

use std::fmt;
use std::collections::HashMap;
use debruijn::DBTerm;

#[derive(Debug,Clone)]
pub enum Term {
    Var(char),
    Abs(char, Box<Term>),
    App(Box<Term>, Box<Term>)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(x) => write!(f, "{}", x),
            Term::Abs(x, t1) => write!(f, "(λ{}. {})", x, t1),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2)
        }
    }
}



impl Term {

    pub fn to_de_bruijn(self) -> DBTerm {
        let mut map = HashMap::new();
        self._to_de_bruijn(&mut map,0)
    }

    fn _to_de_bruijn(self, map: &mut HashMap<char, u32> , c: u32) -> DBTerm {
        match self {
            Term::Var(x) => match map.get(&x) {
                Some(i) => DBTerm::Var(c-i-1),
                None => DBTerm::Var((x as u32 - 'a' as u32)+c+1)
            }
            Term::Abs(x,t) => {
                let before = match map.get(&x) {
                    Some(&x) => Some(x),
                    None => None,
                };
                map.insert(x,c);
                let dbt = t._to_de_bruijn(map, c+1);
                match before {
                    None => map.remove(&x),
                    Some(u) => map.insert(x,u),
                };
                DBTerm::Abs(x as u32 - 'a' as u32, Box::new(dbt))
            }
            Term::App(t1,t2) => DBTerm::App(Box::new(t1._to_de_bruijn(map,c)),
                                            Box::new(t2._to_de_bruijn(map,c)))
            
        }
    }

}
