use std::borrow::Cow;

use nom::{IResult as NomResult, branch::alt, bytes::complete::tag, character::complete::{alpha1, alphanumeric1, crlf, digit0, multispace0}, combinator::{map, recognize}, error::{context, ParseError, VerboseError}, multi::{many0, separated_list0, separated_list1}, sequence::{delimited, pair, separated_pair, tuple}};
pub mod strings;
pub mod types;

use types::*;

type IResult<T, U> = NomResult<T, U, VerboseError<T>>;

use self::strings::parse_string;

pub fn parse_mi_output(input: &str) -> IResult<&str, Output> {
    todo!()
}

fn result_record(input: &str) -> IResult<&str, AsyncOutput> {
    todo!()
}

fn exec_async_output(input: &str) -> IResult<&str, AsyncOutput> {
    let parser = tuple((token, tag("*"), async_output, crlf));
    todo!()
}

fn async_output(input: &str) -> IResult<&str, (AsyncOutputClass, Vec<Variable>)> {
    let parser = tuple((async_class, tag(","), result_list));
    map(parser, |v| (v.0, v.2))(input)
}

// TODO: It's let's wasteful to use Option instead of an empty Vec
fn result_list(input: &str) -> IResult<&str, Vec<Variable>> {
    context("result_list", separated_list1(tag(","), variable))(input)
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

fn token(input: &str) -> IResult<&str, Option<Token>> {
    map(digit0, |v: &str| match v.parse::<u32>() {
        Ok(x) => Some(Token(x)),
        Err(_) => None,
    })(input)
}

fn variable(input: &str) -> IResult<&str, Variable> {
    let parser = context("variable", separated_pair(identifier, tag("="), value));
    map(parser, |v| Variable(v.0, v.1))(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    context(
        "identifier",
        recognize(pair(
            alt((alpha1, tag("_"), tag("-"))),
            many0(alt((alphanumeric1, tag("_"), tag("-")))),
        )),
    )(input)
}

fn value(input: &str) -> IResult<&str, Value> {
    context("value", alt((constant, tuple_value, list)))(input)
}

fn constant(input: &str) -> IResult<&str, Value> {
    match context("constant", parse_string)(input) {
        Ok((rest, x)) => Ok((rest, Value::Const(Cow::from(x)))),
        Err(x) => Err(x),
    }
}

fn tuple_value(input: &str) -> IResult<&str, Value> {
    let parser = context(
        "tuple_value",
        delimited(tag("{"), separated_list0(tag(","), variable), tag("}")),
    );
    match map(parser, |v| TupleValue::from(v))(input) {
        Ok((r, t)) => Ok((r, Value::Tuple(t))),
        Err(x) => Err(x),
    }
}

fn list(input: &str) -> IResult<&str, Value> {
    let variable_list = context("variable_list",map(separated_list0(tag(","), variable), |v| ListValue::from(v)));
    let value_list = context("value_list", map(separated_list0(tag(","), value), |v| ListValue::from(v)));
    let variable_or_value = alt((variable_list, value_list));
    let parser = context("list", delimited(tag("["), variable_or_value, tag("]")));
    map(parser, |v| Value::List(v))(input)
}

fn termination(input: &str) -> IResult<&str, ()> {
    match tag("(gdb)")(input) {
        Ok((rest, _)) => Ok((rest, ())),
        Err(x) => Err(x),
    }
}

// Trim whitespace
pub fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> NomResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    let c = it.next().unwrap_or_default();
                    index += c.len_utf8();
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(nom::Err::Error(nom::error::Error::from_error_kind(
                i,
                nom::error::ErrorKind::TakeUntil,
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::error::convert_error;

    use crate::parser::strings::parse_string;

    use super::*;
    use std::{borrow::Cow, fmt::Debug};

    macro_rules! do_test_result {
        ($input:ident,$comb:expr,$test:expr) => {
            match $comb {
                Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                    println!("{}", convert_error($input, e))
                }
                Err(nom::Err::Incomplete(e)) => println!("{:?}", e),
                Ok(data) => assert_eq!(data, $test),
            }
        };
        ($input:ident,$comb:expr) => {
            match $comb {
                Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                    println!("Error {}", convert_error($input, e))
                }
                Err(nom::Err::Incomplete(e)) => println!("Incomplete {:?}", e),
                Ok(data) => {
                    println!("{:?}", data);
                    panic!("needs test");
                }
            }
        };
    }

    #[test]
    fn test_async_output() {
        let data = "stopped,reason=\"breakpoint-hit\",disp=\"keep\",bkptno=\"1\",thread-id=\"0\",\
                    frame={addr=\"0x08048564\",func=\"main\",args=[{name=\"argc\",value=\"1\"},\
                    {name=\"argv\",value=\"0xbfc4d4d4\"}],file=\"myprog.c\",fullname=\"/home/\
                    nickrob/myprog.c\",line=\"68\",arch=\"i386:x86_64\"}";
        assert_eq!(
            async_output(data).unwrap(),
            ("", (AsyncOutputClass::Stopped, Vec::new()))
        )
    }

    #[test]
    fn test_result_list() {
        let data = "frame={addr=\"0x08048564\",func=\"main\",args=[{name=\"argc\",value=\"1\"},\
                    {name=\"argv\",value=\"0xbfc4d4d4\"}],file=\"myprog.c\",fullname=\"/home/\
                    nickrob/myprog.c\",line=\"68\",arch=\"i386:x86_64\"},data=\"1\",";
        do_test_result!(data, result_list(data))
    }

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
    fn test_list_tuple(){
        let data ="[{name=\"argc\",value=\"1\"},{name=\"argv\",value=\"0xbfc4d4d4\"}]";
        do_test_result!(data,list(data))
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
        let data = "{type=\"breakpoint\"";
        let result = Value::Tuple(TupleValue::Data(vec![Variable(
            "type",
            Value::Const(Cow::from("breakpoint")),
        )]));
        assert_eq!(tuple_value(data).unwrap(), ("", result))
    }

    #[test]
    fn test_empty_tuple() {
        assert_eq!(
            tuple_value("{}").unwrap(),
            ("", Value::Tuple(TupleValue::Empty))
        )
    }

    #[test]
    fn test_variable() {
        let data = "args=[{name=\"argc\",value=\"1\"},\
                    {name=\"argv\",value=\"0xbfc4d4d4\"}]";


        do_test_result!(data,variable(data))
    }

    #[test]
    fn test_parse_string() {
        let data = "\"/home/nikita/pepe.c\"";
        let x = match parse_string::<nom::error::VerboseError<&str>>(data) {
            Ok(x) => Ok(x),
            Err(y) => Err(y),
        };
        println!("{:?}", x)
    }
}
