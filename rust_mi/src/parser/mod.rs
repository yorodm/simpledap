use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{alpha1, alphanumeric1}, combinator::{map, recognize}, multi::{many0, separated_list0}, sequence::{delimited, pair, separated_pair}};
pub mod types;

use types::*;

pub fn parse_mi_output(input: &str) -> IResult<&str, Output> {
    todo!()
}

fn result_class(input: &str) -> IResult<&str, ResultClass> {
    todo!()
}

fn async_class(input: &str) -> IResult<&str, AsyncOutputClass> {
    todo!()
}

fn variable(input: &str) -> IResult<&str, Variable> {
    let parser = separated_pair(identifier, tag("="), value);
    map(parser, |v| {
        Variable(v.0.to_owned(),v.1)
    })(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((constant, tuple, list))(input)
}

fn constant(input: &str) -> IResult<&str, Value> {
    todo!()
}

fn tuple(input: &str) -> IResult<&str, Value> {
    let parser = delimited(tag("["), separated_list0(tag(","), variable), tag("]"));
    match map(parser, |v| TupleValue::from(v))(input) {
        Ok((r, t)) => Ok((r, Value::Tuple(t))),
        Err(x) => Err(x),
    }
}

fn list(input: &str) -> IResult<&str, Value> {
    let variable_list = map(separated_list0(tag(","), variable), |v| ListValue::from(v));
    let value_list = map(separated_list0(tag(","), value), |v| ListValue::from(v));
    let variable_or_value = alt((variable_list, value_list));
    let parser = delimited(tag("["), variable_or_value, tag("]"));
    map(parser, |v| Value::List(v))(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::variable;

    #[test]
    fn test_list() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_tuple() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_variable() {
        let data = "type=\"breakpoint\"";
        let x = variable(data);
        assert_eq!(result, 4);
    }
}
