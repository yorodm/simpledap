#[derive(Debug, PartialEq)]
pub enum Output<'a>{
    ResultRecord,
    OOBRecord(OOB<'a>)
}

#[derive(Debug, PartialEq)]
pub enum OOB<'a>{
    StreamRecord(StreamOutput<'a>),
    AsyncRecord
}

#[derive(Debug, PartialEq)]
pub enum AsyncOutput<'a>{
    ExeAsync(AsyncOutputData<'a>),
    StatusAsync(AsyncOutputData<'a>),
    NotifyAsync(AsyncOutputData<'a>)
}

#[derive(Debug,PartialEq)]
pub struct  AsyncOutputData<'a>(Token,AsyncOutputClass,Option<Vec<Variable<'a>>>);

#[derive(Debug, PartialEq)]
pub enum StreamOutput <'a>{
    Console(&'a str),
    Target(&'a str),
    Log(&'a str)
}

#[derive(Debug, PartialEq)]
pub struct Variable<'a>(pub &'a str, pub Value<'a>);

#[derive(Debug, PartialEq)]
pub enum Value<'a>{
    Const(&'a str),
    Tuple(TupleValue<'a>),
    List(ListValue<'a>)
}

impl<'a> From<Vec<Variable<'a>>> for TupleValue<'a>{
    fn from(_: Vec<Variable>) -> Self {
        todo!()
    }
}

impl<'a> From<Vec<Variable<'a>>> for ListValue<'a>{
    fn from(_: Vec<Variable>) -> Self {
        todo!()
    }
}

impl<'a> From<Vec<Value<'a>>> for ListValue<'a>{
    fn from(_: Vec<Value>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum ListValue<'a>{
    Empty,
    ValueList(Vec<Value<'a>>),
    VariableList(Vec<Variable<'a>>)
}

#[derive(Debug, PartialEq)]
pub enum TupleValue<'a>{
    Empty,
    Data(Vec<Variable<'a>>)
}

#[derive(Debug, PartialEq)]
pub enum StreamKind<'a>{
    Console(&'a str),
    Target(&'a str),
    Log(&'a str)
}

#[derive(Debug, PartialEq)]
pub enum ResultClass{
    Done,
    Running,
    Connected,
    Error,
    Exit
}

#[derive(Debug, PartialEq)]
pub enum AsyncOutputClass{
    Stopped,
    Unknown
}


#[derive(Debug, PartialEq)]
pub struct Token(u32);
