use terminal_color_builder::OutputFormatter as tcb;

pub struct Output {

}

impl Output {
    pub fn new() -> Self {
        Self {}
    }

    pub fn output(&mut self, txt: String) {
        print!("{}", tcb::new().fg().green().text_str("$ ").fg().black().text(txt).print());
    }
}