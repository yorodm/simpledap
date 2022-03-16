use std::borrow::Cow;

use nom::{branch::alt,
          bytes::complete::tag,
          character::complete::{alpha1, alphanumeric1},
          combinator::{map, recognize},
          multi::{many0, separated_list0},
          sequence::{delimited, pair, separated_pair},
          IResult};
pub mod strings;
pub mod types;

use types::*;

use self::strings::parse_string;

pub fn parse_mi_output(input: &str) -> IResult<&str, Output> {
    todo!()
}

fn result_class(input: &str) -> IResult<&str, ResultClass> {
    todo!()
}

fn async_class(input: &str) -> IResult<&str, AsyncOutputClass> {
    alt()
}

fn variable(input: &str) -> IResult<&str, Variable> {
    let parser = separated_pair(identifier, tag("="), value);
    map(parser, |v| Variable(v.0, v.1))(input)
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
    match parse_string(input) {
        Ok((rest, x)) => Ok((rest, Value::Const(Cow::from(x)))),
        Err(x) => Err(x),
    }
}

fn tuple(input: &str) -> IResult<&str, Value> {
    let parser = delimited(tag("{"), separated_list0(tag(","), variable), tag("}"));
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

fn termination(input: &str) -> IResult<&str, ()> {
    match tag("(gdb)")(input) {
        Ok((rest, _)) => Ok((rest, ())),
        Err(x) => Err(x),
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::parser::{list, tuple,
                        types::{ListValue, TupleValue, Value, Variable},
                        variable};

    #[test]
    fn test_emtpy_list() {
        let data = "[]";
        let result = Value::List(ListValue::Empty);
        assert_eq!(list(data).unwrap(), ("", result))
    }

    #[test]
    fn test_list_variables() {
        let data = "[type=\"breakpoint\"]";
        let result = Value::List(ListValue::VariableList(vec![Variable(
            "type",
            Value::Const(Cow::from("breakpoint")),
        )]));
        assert_eq!(list(data).unwrap(), ("", result))
    }

    #[test]
    fn test_tuple() {
        let data = "{type=\"breakpoint\"}";
        let result = Value::Tuple(TupleValue::Data(vec![Variable(
            "type",
            Value::Const(Cow::from("breakpoint")),
        )]));
        assert_eq!(tuple(data).unwrap(), ("", result))
    }

    #[test]
    fn test_empty_tuple() {
        assert_eq!(tuple("{}").unwrap(), ("", Value::Tuple(TupleValue::Empty)))
    }

    #[test]
    fn test_variable() {
        let data = "bkpt={number=\"1\",type=\"breakpoint\"}";
        let result = Variable(
            "bkpt",
            Value::Tuple(TupleValue::Data(vec![
                Variable("number", Value::Const(Cow::from("1"))),
                Variable("type", Value::Const(Cow::from("breakpoint"))),
            ])),
        );
        assert_eq!(variable(data).unwrap(), ("", result))
    }
}
