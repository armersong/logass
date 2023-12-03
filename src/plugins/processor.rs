use crate::plugins::color::Color;
use crate::plugins::ignore::Ignore;
use crate::plugins::{Context, TextFilter};
use std::fs::File;
use std::io::{Read, Result};
use crate::log::log;

pub struct Processor {
    plugins: Vec<Box<dyn TextFilter>>,
}

impl Processor {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn init(&mut self, plugin_config_path: &str) -> std::io::Result<()> {
        let mut ignore: Box<dyn TextFilter> = Box::new(Ignore::new());
        Self::init_plugin(plugin_config_path, &mut ignore)?;
        self.plugins.push(ignore);

        let mut color: Box<dyn TextFilter> = Box::new(Color::new());
        Self::init_plugin(plugin_config_path, &mut color)?;
        self.plugins.push(color);

        self.plugins.sort_by(|a, b| a.order().cmp(&b.order()));
        Ok(())
    }

    fn init_plugin(plugin_config_path: &str, plugin: &mut Box<dyn TextFilter>) -> Result<()> {
        let config =
            Self::read_config(format!("{}/{}.json", plugin_config_path, plugin.name()).as_str())?;
        log(format!("init plugin {}", plugin.name()));
        plugin.init(config.as_str())?;
        Ok(())
    }

    fn read_config(filename: &str) -> Result<String> {
        let mut file = File::open(filename)?;
        let mut config = String::new();
        file.read_to_string(&mut config)?;
        Ok(config)
    }

    pub fn handle(&mut self, text: String) -> Option<String> {
        let mut ctx = Context {};
        let mut txt = Some(text);
        for p in self.plugins.iter_mut() {
            if let Some(t) = txt.take() {
                if let Some(t1) = p.filter(&mut ctx, t) {
                    txt = Some(t1);
                }
            } else {
                return None;
            }
        }
        txt
    }
}
