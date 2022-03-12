
#[derive(Debug)]
pub struct Parser<'a>{
    input: &'a str
}

impl <'a> Parser<'a>{
    pub fn from(data: &'a str) -> Parser<'a>{
        Parser{
            input: data
        }
    }
}
