use termcolor::{ColorChoice, StandardStream};
use std::io::{Write};

pub struct Output {
    out: StandardStream,
}

impl Output {
    pub fn new() -> Self {
        let stdout = StandardStream::stdout(ColorChoice::Always);
        Self { out: stdout }
    }

    pub fn output(&mut self, txt: String) {
        let _= self.out.write(txt.as_bytes());
    }
}
