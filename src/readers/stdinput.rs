use crate::readers::TextReader;

pub struct StdInput {

}

impl StdInput {
    pub fn new() -> Self {
        Self {}
    }
}

impl TextReader for StdInput {
    fn read_line(&mut self) -> Option<String> {
        let mut buff = String::new();
        if let Ok(_s) = std::io::stdin().read_line(&mut buff) {
            return Some(buff);
        }
        None
    }
}