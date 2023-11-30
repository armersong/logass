use crate::plugins::TextFilter;

pub struct Processor {
    plugins: Vec<Box<dyn TextFilter>>,
}

impl Processor {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn init(&mut self, plugin_config_path: &str) -> std::io::Result<()> {
        Ok(())
    }

    pub fn handle(&mut self, text: String) -> Option<String> {
        Some(text)
    }
}