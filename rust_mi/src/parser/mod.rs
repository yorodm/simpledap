use std::borrow::Cow;

use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{alpha1, alphanumeric1, crlf, digit0}, combinator::{map, recognize}, error::Error, multi::{many0, separated_list0}, sequence::{delimited, pair, separated_pair, tuple}};
pub mod strings;
pub mod types;

use types::*;

use self::strings::parse_string;

pub fn parse_mi_output(input: &str) -> IResult<&str, Output> {
    todo!()
}

fn result_record(input: &str) -> IResult<&str, AsyncOutput> {
    todo!()
}


fn exec_async_output(input:&str) ->IResult<&str,AsyncOutput> {
    let parser = tuple((
        token,
        tag("*"),
        async_output,
        crlf
    ));
    map(parser, |v|{

    })(input)
}

fn async_output(input: &str) -> IResult<&str, (AsyncOutputClass, Option<Vec<Variable>>)> {
     tuple((async_class,result_list))(input)
}

fn result_list(input:&str) -> IResult<&str, Option<Vec<Variable>>>{
    todo!();

}

fn result_class(input: &str) -> IResult<&str, ResultOutputClass> {
    let done = map(tag("done"), |_| ResultOutputClass::Done);
    let running = map(tag("running"), |_| ResultOutputClass::Running);
    let connected = map(tag("connected"), |_| ResultOutputClass::Connected);
    let error = map(tag("error"), |_| ResultOutputClass::Error);
    let exit = map(tag("exit"), |_| ResultOutputClass::Exit);
    alt((done, running, connected, error, exit))(input)
}

fn async_class(input: &str) -> IResult<&str, AsyncOutputClass> {
    let stopped = map(tag("stopped"), |_| AsyncOutputClass::Stopped);
    let unknown = map(alpha1, |_| AsyncOutputClass::Unknown);
    alt((stopped, unknown))(input)
}

fn token(input:&str) -> IResult<&str,Option<Token>> {
    map(digit0, |v| {
        match v.parse::<u32>(){
            Ok(x) => Some(Token(x)),
            Err(y) => None
        }
    })(input)
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
    alt((constant, tuple_value, list))(input)
}

fn constant(input: &str) -> IResult<&str, Value> {
    match parse_string(input) {
        Ok((rest, x)) => Ok((rest, Value::Const(Cow::from(x)))),
        Err(x) => Err(x),
    }
}

fn tuple_value(input: &str) -> IResult<&str, Value> {
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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::parser::types::{ListValue, ResultOutputClass, TupleValue, Value, Variable};
    use crate::parser::*;

    #[test]
    fn test_async_class() {
        // TODO: test the other fields
        let data = "stopped";
        let result = AsyncOutputClass::Stopped;
        assert_eq!(async_class(data).unwrap(), ("", result));
        let data = "whatever";
        let result = AsyncOutputClass::Unknown;
        assert_eq!(async_class(data).unwrap(), ("", result))
    }

    #[test]
    fn test_result_class() {
        // TODO: test the other fields
        let data = "done";
        let result = ResultOutputClass::Done;
        assert_eq!(result_class(data).unwrap(), ("", result))
    }

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
        assert_eq!(tuple_value(data).unwrap(), ("", result))
    }

    #[test]
    fn test_empty_tuple() {
        assert_eq!(tuple_value("{}").unwrap(), ("", Value::Tuple(TupleValue::Empty)))
    }

    #[test]
    fn test_variable() {
        let data = "type=\"breakpoint\"";
        let result = Variable("type", Value::Const(Cow::from("breakpoint")));
        assert_eq!(variable(data).unwrap(), ("", result))
    }
}
