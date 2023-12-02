use crate::plugins::color::Color;
use crate::plugins::{Context, TextFilter};

pub struct Processor {
    plugins: Vec<Box<dyn TextFilter>>,
}

impl Processor {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn init(&mut self, _plugin_config_path: &str) -> std::io::Result<()> {
        let mut p: Box<dyn TextFilter> = Box::new(Color::new());
        p.init("")?;
        self.plugins.push(p);
        Ok(())
    }

    pub fn handle(&mut self, text: String) -> Option<String> {
        let mut ctx = Context{};
        let mut txt = Some(text);
        for p in self.plugins.iter_mut() {
            if let Some(t) = txt.take() {
                if let Some(t1) = p.filter(&mut ctx, t) {
                    txt = Some(t1);
                }
            } else {
                break;
            }
        }
        txt
    }
}