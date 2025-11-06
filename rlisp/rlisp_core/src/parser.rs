use crate::types::Expr;
use crate::types::Expr::*;

use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;

pub(crate) fn parse_form(input: &str) -> Result<(&str, Expr), nom::Err<nom::error::Error<&str>>> {
    delimited(
        space0,
        delimited(char('('), parse_math_expr, char(')')),
        space0,
    )(input)
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    let (input, num1) = parse_term(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_factor)))(input)?;
    let (input, num1) = parse_factor(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, exprs) = many0(tuple((char('^'), parse_factor)))(input)?;
    let (input, num1) = parse_operation(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    alt((parse_parens, parse_number))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        space0,
        delimited(char('('), parse_math_expr, char(')')),
        space0,
    )(input)
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => EAdd(Box::new(expr2), Box::new(expr1)),
        '-' => ESub(Box::new(expr2), Box::new(expr1)),
        '*' => EMul(Box::new(expr2), Box::new(expr1)),
        '/' => EDiv(Box::new(expr2), Box::new(expr1)),
        '^' => EExp(Box::new(expr2), Box::new(expr1)),
        _ => panic!("Unknown Operation"),
    }
}

fn parse_enum(parsed_num: &str) -> Expr {
    let num = f32::from_str(parsed_num).unwrap();
    ENum(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_enum)(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_form;
    use crate::types::Expr::*;

    #[test]
    fn parse_one_sexp_form() {
        let parsed = parse_form("(+ 2 4)");
        assert_eq!(
            parsed,
            Ok(("", EAdd(Box::new(ENum(2.0)), Box::new(ENum(4.0)))))
        );
    }

    #[test]
    fn parse_nested_sexp_at_1_form() {
        let parsed = parse_form("(+ (+ 2 2) 2)");
        assert_eq!(
            parsed,
            Ok((
                "",
                EAdd(
                    Box::new(EAdd(Box::new(ENum(2.0)), Box::new(ENum(2.0)))),
                    Box::new(ENum(2.0))
                )
            ))
        );
    }

    #[test]
    fn parse_nested_sexp_at_2_form() {
        let parsed = parse_form("(+ 2 (+ 2 2))");
        assert_eq!(
            parsed,
            Ok((
                "",
                EAdd(
                    Box::new(ENum(2.0)),
                    Box::new(EAdd(Box::new(ENum(2.0)), Box::new(ENum(2.0)))),
                )
            ))
        );
    }

    #[test]
    fn parse_polish_add() {
        let parsed = parse_form("(+ 2 4)");
        assert_eq!(
            parsed,
            Ok(("", EAdd(Box::new(ENum(2.0)), Box::new(ENum(4.0)))))
        );
    }

    #[test]
    fn parse_polish_statement() {
        let parsed = parse_form("(+ (* 2 (/ 40 20))4)");
        assert_eq!(
            parsed,
            Ok((
                "",
                EAdd(
                    Box::new(EMul(
                        Box::new(ENum(2.0)),
                        Box::new(EDiv(Box::new(ENum(40.0)), Box::new(ENum(20.0))))
                    )),
                    Box::new(ENum(4.0))
                )
            ))
        );
    }
}
