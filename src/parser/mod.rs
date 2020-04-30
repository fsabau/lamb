pub mod ast;

use std::iter::FromIterator;
use std::path::PathBuf;
use std::str::FromStr;
use crate::error::LambError;
use ast::{Expr, Statement};
use nom::{
    bytes::complete::tag, 
    error::{
        ParseError,
        VerboseError,
        convert_error,
    },
    AsChar,
    InputTakeAtPosition,
    IResult,
    sequence::{
        preceded,
        delimited,
        terminated,
    },
    character::complete::{
        alphanumeric1,
        anychar,
        char,
        multispace0,
        multispace1,
        digit1,
    }, 
    combinator::{
        map_res,
        all_consuming,
        not,
        peek,
        map,
        verify,
    },
    branch::alt,
    multi::{
        many0,
        separated_list,
    }
};

fn lowercase(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    verify(anychar, |c| c.is_lowercase())(i)
}

// fn uppercase(i: &str) -> IResult<&str, char> {
//     verify(anychar, |c| c.is_uppercase())(i)
// }


fn space<I,O,E>(f: impl Fn(I)->IResult<I,O,E>) -> impl Fn(I) -> IResult<I, O, E> 
where I: InputTakeAtPosition,
      <I as InputTakeAtPosition>::Item: AsChar + Clone,
      E: ParseError<I> {
    preceded(multispace0,f)
}

fn var(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = not(peek(tag("let")))(i)?;
    map(space(lowercase), |c| Expr::Var(c))(i)
}

fn abs(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i,_) = char('\\')(i)?;
    let (i, c) = verify(space(anychar), |c| c.is_lowercase())(i)?;
    let (i,_) = char('.')(i)?;
    map(space(expr), move |t| Expr::Abs(c,Box::new(t)))(i)
} 

fn apps_vec(i: &str) -> IResult<&str, Vec<Expr>, VerboseError<&str>> {
    verify(separated_list(multispace1, expr_no_app), |v: &Vec<Expr>| v.len()>=2)(i)
}

fn app(i: &str) -> IResult<&str,Expr, VerboseError<&str>> {
    map(apps_vec, 
        |v: Vec<Expr>| v.iter()
                  .skip(1)
                  .fold(v[0].clone(), |t, x| Expr::App(Box::new(t), Box::new( x.clone() )))
    )(i)
}

fn verify_name(name: &str) -> bool {
    if name == "let" {
        return false
    }

    (name.starts_with(char::is_lowercase) && name.len() >1 ) || name.starts_with(char::is_uppercase)
}

fn identifier(i: &str) -> IResult<&str,String, VerboseError<&str>> {
    let (i, _) = not(peek(tag("let")))(i)?;
    map(verify(alphanumeric1, verify_name), |s: &str| s.to_owned())(i)
}

fn number(i: &str) -> IResult<&str, u64, VerboseError<&str>> {
    map_res(digit1, u64::from_str)(i)
}


fn expr_no_app(i: &str) -> IResult<&str,Expr, VerboseError<&str>> {
    space(
        alt((
            delimited(
                char('('),
                expr,
                space(char(')'))
                ),
            abs, 
            map(number, Expr::Num),
            map(identifier, |s| Expr::Ident(s)),
            var
        ))
    )(i)
}

pub fn expr(i: &str) -> IResult<&str,Expr, VerboseError<&str>> {
    space(
        alt((
            app,
            delimited(
                char('('),
                expr,
                char(')')
                ),
            abs,
            map(number, Expr::Num),
            map(identifier, Expr::Ident),
            var
        ))
    )(i)
}

fn path(i: &str) -> IResult<&str, PathBuf, VerboseError<&str>> {
    map(many0(verify(anychar, |c| !c.is_whitespace())),
        |s| PathBuf::from(String::from_iter(s)))(i)

}

fn expr_stmt(i: &str) -> IResult<&str,Statement, VerboseError<&str>> {
    map(expr, |e| Statement::Expr(e))(i)
}

fn import_stmt(i: &str) -> IResult<&str,Statement, VerboseError<&str>> {
    let (i,_) = space(tag("import"))(i)?;
    map(preceded(multispace1, path), |mut p| {
        p.set_extension("lamb");
        Statement::Import(p)
    })(i)
}

fn let_stmt(i: &str) -> IResult<&str,Statement, VerboseError<&str>> {
    let (i,_) = space(tag("let"))(i)?;
    let (i, ident) = delimited(multispace1, identifier, space(char('=')))(i)?;
    map(expr, move |t| Statement::Let(ident.clone(),t))(i) 
} 

fn stmt(i: &str) -> IResult<&str,Statement, VerboseError<&str>> {
    alt((let_stmt,import_stmt,expr_stmt))(i)
}

pub fn repl(i: &str) -> IResult<&str,Statement, VerboseError<&str>> {
    all_consuming(terminated(stmt,multispace0))(i) 
}

pub fn file(i: &str) -> IResult<&str,Vec<Statement>, VerboseError<&str>> {
    all_consuming(terminated(many0(alt((let_stmt,import_stmt))),multispace0))(i) 
}

pub fn parse<O>(parser: impl Fn(&str) -> IResult<&str, O, VerboseError<&str>>, input: &str) -> Result<(&str,O), LambError>{
    match parser(input) {
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(LambError::Parse(convert_error(input,e))),
        Ok(p) => Ok(p),
        _ => panic!("Impossible"),
    }
}



