use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum Output<'a> {
    ResultRecord(OutputData<'a>),
    OOBRecord(OOB<'a>),
}

#[derive(Debug, PartialEq)]
pub enum OOB<'a> {
    StreamRecord(StreamOutput<'a>),
    AsyncRecord(AsyncOutput<'a>),
}

#[derive(Debug, PartialEq)]
pub enum AsyncOutput<'a> {
    ExeAsync(OutputData<'a>),
    StatusAsync(OutputData<'a>),
    NotifyAsync(OutputData<'a>),
}

#[derive(Debug, PartialEq)]
pub struct OutputData<'a>(pub Option<Token>, pub OutputClass, pub Vec<Variable<'a>>);

#[derive(Debug, PartialEq)]
pub enum StreamOutput<'a> {
    Console(&'a str),
    Target(&'a str),
    Log(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Variable<'a>(pub &'a str, pub Value<'a>);


#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Const(Cow<'a, str>),
    Tuple(TupleValue<'a>),
    List(ListValue<'a>),
}


impl<'a>  From<&'a str> for Value<'a> {
    fn from(f: &'a str) -> Self {
        Value::Const(Cow::from(f))
    }
}

impl<'a> From<Vec<Variable<'a>>> for TupleValue<'a> {
    fn from(v: Vec<Variable<'a>>) -> Self {
        if v.is_empty() {
            TupleValue::Empty
        } else {
            TupleValue::Data(v)
        }
    }
}

impl<'a> From<Vec<Variable<'a>>> for ListValue<'a> {
    fn from(v: Vec<Variable<'a>>) -> Self {
        if v.is_empty() {
            ListValue::Empty
        } else {
            ListValue::VariableList(v)
        }
    }
}

impl<'a> From<Vec<Value<'a>>> for ListValue<'a> {
    fn from(v: Vec<Value<'a>>) -> Self {
        ListValue::ValueList(v)
    }
}

impl<'a> Default for ListValue<'a> {
    fn default() -> Self {
        ListValue::Empty
    }
}

impl<'a> Default for TupleValue<'a> {
    fn default() -> Self {
        TupleValue::Empty
    }
}

#[derive(Debug, PartialEq)]
pub enum ListValue<'a> {
    Empty,
    ValueList(Vec<Value<'a>>),
    VariableList(Vec<Variable<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum TupleValue<'a> {
    Empty,
    Data(Vec<Variable<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum StreamKind<'a> {
    Console(&'a str),
    Target(&'a str),
    Log(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum OutputClass {
    Done,
    Running,
    Connected,
    Error,
    Exit,
    Stopped,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token(pub u32);
