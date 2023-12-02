use crate::readers::stdinput::StdInput;

pub mod stdinput;
pub mod filesource;

pub trait TextReader {
    fn read_line(&mut self) -> Option<String>;
}

pub fn build_reader() -> impl TextReader {
    StdInput::new()
}
