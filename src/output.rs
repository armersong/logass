pub struct Output {}

impl Output {
    pub fn new() -> Self {
        Self {}
    }

    pub fn output(&mut self, txt: String) {
        print!("{}", txt);
    }
}
