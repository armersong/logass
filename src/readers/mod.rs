use crate::readers::stdinput::StdInput;

mod stdinput;

pub trait TextReader {
    fn read_line(&mut self) -> Option<String>;
}

pub fn build_reader() -> impl TextReader {
    StdInput::new()
}
