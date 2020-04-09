use crate::term::Term;
use nom::{
    bytes::complete::tag, 
    error::ParseError,
    AsChar,
    InputTakeAtPosition,
    IResult,
    sequence::{
        preceded,
        separated_pair,
        delimited
    },
    character::complete::{
        anychar,
        char,
        multispace0,
        multispace1,
    }, 
    combinator::{
        map,
        verify,
    },
    branch::alt,
    multi::{
        many0,
        many_m_n,
        separated_list,
    }
};

fn lowercase(i: &str) -> IResult<&str, char> {
    verify(anychar, |c| c.is_lowercase())(i)
}

fn uppercase(i: &str) -> IResult<&str, char> {
    verify(anychar, |c| c.is_uppercase())(i)
}


fn space<I,O,E>(f: impl Fn(I)->IResult<I,O,E>) -> impl Fn(I) -> IResult<I, O, E> 
where I: InputTakeAtPosition,
      <I as InputTakeAtPosition>::Item: AsChar + Clone,
      E: ParseError<I> {
    preceded(multispace0,f)
}

fn var(i: &str) -> IResult<&str, Term> {
    map(space(lowercase), |c| Term::Var(c as u32 - 'a' as u32))(i)
}

fn abs(i: &str) -> IResult<&str, Term> {
    let (i,_) = char('\\')(i)?;
    let (i, c) = verify(space(anychar), |c| c.is_lowercase())(i)?;
    let (i,_) = char('.')(i)?;
    map(space(term), move |t| Term::Abs(c,Box::new(t)))(i)
} 

fn apps_vec(i: &str) -> IResult<&str, Vec<Term>> {
    verify(separated_list(multispace1, term_no_app), |v: &Vec<Term>| v.len()>=2)(i)
}

fn app(i: &str) -> IResult<&str,Term> {
    map(apps_vec, 
        |v: Vec<Term>| v.iter()
                  .skip(1)
                  .fold(v[0].clone(), |t, x| Term::App(Box::new(t), Box::new( x.clone() )))
    )(i)
}

fn identifier(i: &str) -> IResult<&str,String> {
    map(many_m_n(2,10000, lowercase), |v| v.iter().collect::<String>())(i)
}

fn let_expr(i: &str) -> IResult<&str,(String,Term)> {
    let (i,_) = tag("let")(i)?;
    let (i, ident) = delimited(multispace1, identifier, space(char('=')))(i)?;
    map(term, move |t| (ident.clone(),t))(i) 
} 

fn file(i: &str) -> IResult<&str,Vec<(String,Term)>> {
    many0(let_expr)(i) 
}

fn term_no_app(i: &str) -> IResult<&str,Term> {
    space(
        alt((
            delimited(
                char('('),
                term,
                space(char(')'))
                ),
            abs, 
            var
        ))
    )(i)
}

pub fn term(i: &str) -> IResult<&str,Term> {
    space(
        alt((
            app,
            delimited(
                char('('),
                term,
                char(')')
                ),
            abs,
            var
        ))
    )(i)
}
