pub mod parser;
pub mod types;
pub mod commands;

#[derive(Debug)]
pub struct MIController {

}

impl Default for MIController {
    fn default() -> Self {
        Self {  }
    }
}

impl MIController {
    pub fn new() -> MIController{
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
