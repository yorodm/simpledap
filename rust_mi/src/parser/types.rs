#[derive(Debug, PartialEq)]
pub enum Output{
    ResultRecord,
    OOBRecord(OOB)
}

#[derive(Debug, PartialEq)]
pub enum OOB{
    StreamRecord(StreamOutput),
    AsyncRecord
}

#[derive(Debug, PartialEq)]
pub enum AsyncOutput{
    ExeAsync(AsyncOutputData),
    StatusAsync(AsyncOutputData),
    NotifyAsync(AsyncOutputData)
}

#[derive(Debug,PartialEq)]
pub struct  AsyncOutputData(Token,AsyncOutputClass,Option<Vec<Variable>>);

#[derive(Debug, PartialEq)]
pub enum StreamOutput{
    Console(String),
    Target(String),
    Log(String)
}

#[derive(Debug, PartialEq)]
pub struct Variable(pub String, pub Value);

#[derive(Debug, PartialEq)]
pub enum Value{
    Const(String),
    Tuple(TupleValue),
    List(ListValue)
}

impl From<Vec<Variable>> for TupleValue{
    fn from(_: Vec<Variable>) -> Self {
        todo!()
    }
}

impl From<Vec<Variable>> for ListValue{
    fn from(_: Vec<Variable>) -> Self {
        todo!()
    }
}

impl From<Vec<Value>> for ListValue{
    fn from(_: Vec<Value>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum ListValue{
    Empty,
    ValueList(Vec<Value>),
    VariableList(Vec<Variable>)
}

#[derive(Debug, PartialEq)]
pub enum TupleValue{
    Empty,
    Data(Vec<Variable>)
}

#[derive(Debug, PartialEq)]
pub enum StreamKind{
    Console(String),
    Target(String),
    Log(String)
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
