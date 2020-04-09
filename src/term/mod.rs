pub mod evaluate;

use std::fmt;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum Term {
    Var(u32),
    Abs(char, Box<Term>),
    App(Box<Term>, Box<Term>)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(x) => write!(f, "{}", to_char(*x)),
            Term::Abs(x, t1) => write!(f, "(λ{}. {})", x, t1),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2)
        }
    }
}

fn to_idx(c: char) -> u32 {
    (c as u8 - 'a' as u8) as u32
}

fn to_char(i: u32) -> char {
    (i as u8 + 'a' as u8) as char
}

impl Term {

    pub fn normal_order(self) -> Term {
        match self {
            Term::Var(x) => Term::Var(x),
            Term::Abs(x,t1) => Term::Abs(x,Box::new((*t1).normal_order())),
            Term::App(t1,t2) => match *t1 {
                Term::Abs(_,_) => Term::App(t1,t2).beta().normal_order(),
                _              => Term::App(Box::new(t1.normal_order()),
                                            Box::new(t2.normal_order())).normal_order()
            }        
        }

    }

    pub fn beta(self) -> Term {
        match self { 
            Term::App(t1,t2) => match *t1 {
                Term::Abs(_, t3) => t3.sub(0,&t2.shift(1)).shift(-1),
                _ => Term::App(t1,t2)
            },
            t => t
        }
    }

    pub fn sub(self, y: u32, s: &Term ) -> Term {
        self._sub(y,s,0)
    }

    
    fn _sub(self, y: u32, s: &Term, c: u32) -> Term {
        match self {
            Term::Var(x) => match x==y+c { 
                true  => s.clone().shift(c as i32),
                false => Term::Var(x) 
            },
            Term::Abs(x,t1) => Term::Abs(x, Box::new(t1._sub(y, s,c+1) )),
            Term::App(t1,t2) => Term::App(Box::new(t1._sub(y,s,c)), 
                                          Box::new(t2._sub(y,s,c)))
        }
    }

    pub fn shift(self, d: i32) -> Term {
        self._shift(d,0)
    }
    
    fn _shift(self, d: i32, cutoff: u32) -> Term {
        match self {
            Term::Var(x) =>  match x >= cutoff  { 
                true  => Term::Var((x as i32 + d) as u32),
                false => Term::Var(x) 
            },
            Term::Abs(x,t1) => Term::Abs(x, Box::new(t1._shift(d, cutoff+1))),
            Term::App(t1,t2) => Term::App(Box::new(t1._shift(d,cutoff)),
                                          Box::new(t2._shift(d,cutoff))) 
        }
    }
    
    pub fn to_de_bruijn(&mut self) {
        let mut map = HashMap::new();
        self._to_de_bruijn(&mut map,0);
    }

    fn _to_de_bruijn(&mut self, map: &mut HashMap<char, u32> , c: u32) {
        match self {
            Term::Var(x) => match map.get(&to_char(*x)) {
                Some(i) => *x=c-i-1,
                None => *x=*x+c+1
            }
            Term::Abs(x,t) => {
                map.insert(*x,c);
                t._to_de_bruijn(map, c+1);
            }
            Term::App(t1,t2) => {
                t1._to_de_bruijn(map,c);
                t2._to_de_bruijn(map,c);
            }
        }
    }

    pub fn to_chars(&mut self) {
        let mut vec = Vec::new();
        self._to_chars(&mut vec,0);
    }

    fn _to_chars(&mut self, vec: &mut Vec<char> , c: u32) {
        match self {
            Term::Var(x) => match *x >= c {
                true => *x = *x-c-1,
                false => *x = to_idx(vec[(c - 1 - *x) as usize])
            }
            Term::Abs(x,t) => {
                vec.push(*x);
                t._to_chars(vec, c+1);
            }
            Term::App(t1,t2) => {
                t1._to_chars(vec,c);
                t2._to_chars(vec,c);
            }
        }
    }
}

