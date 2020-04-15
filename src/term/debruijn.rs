
use super::evaluate::Strategy;
use std::fmt;
use super::Term;

#[derive(Debug,Clone)]
pub enum DBTerm {
    Var(u32),
    Abs(u32, Box<DBTerm>),
    App(Box<DBTerm>, Box<DBTerm>)
}

impl fmt::Display for DBTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBTerm::Var(x) => write!(f, "{}", x),
            DBTerm::Abs(_, t1) => write!(f, "(λ {})", t1),
            DBTerm::App(t1, t2) => write!(f, "({} {})", t1, t2)
        }
    }
}


fn to_char(i: u32) -> char {
    (i as u8 + 'a' as u8) as char
}




impl DBTerm {

    pub fn reduce(self, strategy: Strategy) -> DBTerm {
        self._reduce(false, strategy)
    }

    fn _reduce(self, reduced:bool, strategy: Strategy) -> DBTerm {
        match self {
            DBTerm::Var(x) => DBTerm::Var(x),

            DBTerm::Abs(x,t1) => match strategy {
                Strategy::NormalOrder | Strategy::ApplicativeOrder => DBTerm::Abs(x, Box::new(t1._reduce(false,strategy))),
                Strategy::CallByValue | Strategy::CallByName       => DBTerm::Abs(x,t1),
            }

            DBTerm::App(t1,t2) => match *t1 {
                DBTerm::Abs(_,_) => match strategy {
                    Strategy::NormalOrder      | Strategy::CallByName  => DBTerm::App(t1,t2).beta()._reduce(false, strategy),
                    Strategy::ApplicativeOrder | Strategy::CallByValue => DBTerm::App(Box::new(t1._reduce(false, strategy)),
                                                                                    Box::new(t2._reduce(false,strategy)))
                                                                          .beta()._reduce(false,strategy),
                }
                t1 => match reduced {
                    true  => DBTerm::App(Box::new(t1),t2),
                    false => DBTerm::App(Box::new(t1._reduce(false,strategy)),
                                       Box::new(t2._reduce(false,strategy)))._reduce(true,strategy),
                }
            }
        }
    }

    fn _one_step(self, strategy: Strategy) -> (DBTerm, bool) {
        match self {
            DBTerm::Var(x) => (DBTerm::Var(x), false),
            DBTerm::Abs(x,t1) => match strategy {
                Strategy::NormalOrder | Strategy::ApplicativeOrder => {
                    let (t, b) = t1._one_step(strategy);
                    (DBTerm::Abs(x, Box::new(t)), b)
                }
                Strategy::CallByValue | Strategy::CallByName       => (DBTerm::Abs(x,t1),false),
            }
            DBTerm::App(t1,t2) => match strategy {
                Strategy::NormalOrder | Strategy::CallByName => {
                    if let DBTerm::Abs(_,_) = *t1 {
                        return (DBTerm::App(t1,t2).beta(),true)
                    }

                    let (t1,b) = t1._one_step(strategy);
                    if b { return (DBTerm::App(Box::new(t1), t2),true) }
                    let (t2,b) = t2._one_step(strategy);
                    if b { return (DBTerm::App(Box::new(t1), Box::new(t2)),true) }
                    (DBTerm::App(Box::new(t1),Box::new(t2)), false)
                }
                Strategy::ApplicativeOrder | Strategy::CallByValue => {
                    let (t1,b) = t1._one_step(strategy);
                    if b { return (DBTerm::App(Box::new(t1), t2), true) }
                    let (t2,b) = t2._one_step(strategy);
                    if b { return (DBTerm::App(Box::new(t1), Box::new(t2)), true) }

                    if let DBTerm::Abs(_,_) = t1 {
                        return (DBTerm::App(Box::new(t1), Box::new(t2)).beta(), true)
                    }
                    (DBTerm::App(Box::new(t1),Box::new(t2)), false)
                }
            }
        }
    }
                
    pub fn many_steps(self, strategy: Strategy) -> DBTerm {
        let (t,b) = self._one_step(strategy);
        match b {
            true => t.many_steps(strategy),
            false => t
        }
    }

    pub fn beta(self) -> DBTerm {
        match self { 
            DBTerm::App(t1,t2) => match *t1 {
                DBTerm::Abs(_, t3) => t3.sub(0,&t2.shift(1)).shift(-1),
                _ => DBTerm::App(t1,t2)
            },
            t => t
        }
    }

    pub fn sub(self, y: u32, s: &DBTerm ) -> DBTerm {
        self._sub(y,s,0)
    }

    
    fn _sub(self, y: u32, s: &DBTerm, c: u32) -> DBTerm {
        match self {
            DBTerm::Var(x) => match x==y+c { 
                true  => s.clone().shift(c as i32),
                false => DBTerm::Var(x) 
            },
            DBTerm::Abs(x,t1) => DBTerm::Abs(x, Box::new(t1._sub(y, s,c+1) )),
            DBTerm::App(t1,t2) => DBTerm::App(Box::new(t1._sub(y,s,c)), 
                                          Box::new(t2._sub(y,s,c)))
        }
    }

    pub fn shift(self, d: i32) -> DBTerm {
        self._shift(d,0)
    }
    
    fn _shift(self, d: i32, cutoff: u32) -> DBTerm {
        match self {
            DBTerm::Var(x) =>  match x >= cutoff  { 
                true  => DBTerm::Var((x as i32 + d) as u32),
                false => DBTerm::Var(x) 
            },
            DBTerm::Abs(x,t1) => DBTerm::Abs(x, Box::new(t1._shift(d, cutoff+1))),
            DBTerm::App(t1,t2) => DBTerm::App(Box::new(t1._shift(d,cutoff)),
                                          Box::new(t2._shift(d,cutoff))) 
        }
    }
    

    pub fn to_term(self) -> Term {
        let mut vec = Vec::new();
        self._to_term(&mut vec,0)
    }

    fn _to_term(self, vec: &mut Vec<char> , c: u32) -> Term {
        match self {
            DBTerm::Var(x) => match x >= c {
                true =>  Term::Var(to_char(x-c-1)),
                false => Term::Var(vec[(c - 1 - x) as usize])
            }
            DBTerm::Abs(x,t) => {
                vec.push(to_char(x));
                let t = t._to_term(vec, c+1);
                vec.pop();
                Term::Abs(to_char(x),Box::new(t))
            }
            DBTerm::App(t1,t2) => Term::App(Box::new(t1._to_term(vec,c)),
                                            Box::new(t2._to_term(vec,c)))
            
        }
    }
}

