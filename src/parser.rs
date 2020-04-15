use crate::expr::Expr;
use nom::{
    bytes::complete::tag, 
    error::ParseError,
    AsChar,
    InputTakeAtPosition,
    IResult,
    sequence::{
        preceded,
        delimited
    },
    character::complete::{
        anychar,
        char,
        multispace0,
        multispace1,
    }, 
    combinator::{
        not,
        peek,
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

fn var(i: &str) -> IResult<&str, Expr> {
    let (i, _) = not(peek(tag("let")))(i)?;
    map(space(lowercase), |c| Expr::Var(c))(i)
}

fn abs(i: &str) -> IResult<&str, Expr> {
    let (i,_) = char('\\')(i)?;
    let (i, c) = verify(space(anychar), |c| c.is_lowercase())(i)?;
    let (i,_) = char('.')(i)?;
    map(space(expr), move |t| Expr::Abs(c,Box::new(t)))(i)
} 

fn apps_vec(i: &str) -> IResult<&str, Vec<Expr>> {
    verify(separated_list(multispace1, expr_no_app), |v: &Vec<Expr>| v.len()>=2)(i)
}

fn app(i: &str) -> IResult<&str,Expr> {
    map(apps_vec, 
        |v: Vec<Expr>| v.iter()
                  .skip(1)
                  .fold(v[0].clone(), |t, x| Expr::App(Box::new(t), Box::new( x.clone() )))
    )(i)
}

fn identifier(i: &str) -> IResult<&str,String> {
    let (i, _) = not(peek(tag("let")))(i)?;
    map(many_m_n(2,10000, lowercase), |v| v.iter().collect::<String>())(i)
}

fn let_expr(i: &str) -> IResult<&str,(String,Expr)> {
    let (i,_) = space(tag("let"))(i)?;
    let (i, ident) = delimited(multispace1, identifier, space(char('=')))(i)?;
    map(expr, move |t| (ident.clone(),t))(i) 
} 

pub fn file(i: &str) -> IResult<&str,Vec<(String,Expr)>> {
    many0(let_expr)(i) 
}

fn expr_no_app(i: &str) -> IResult<&str,Expr> {
    space(
        alt((
            delimited(
                char('('),
                expr,
                space(char(')'))
                ),
            abs, 
            map(identifier, |s| Expr::Ident(s)),
            var
        ))
    )(i)
}

pub fn expr(i: &str) -> IResult<&str,Expr> {
    space(
        alt((
            app,
            delimited(
                char('('),
                expr,
                char(')')
                ),
            abs,
            map(identifier, |s| Expr::Ident(s)),
            var
        ))
    )(i)
}
