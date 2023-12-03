use crate::plugins::model::IgnoreConfig;
use crate::plugins::{Context, TextFilter, HIGHEST_ORDER};
use regex::Regex;
use crate::log::log;

pub struct Ignore {
    rules: Vec<Regex>,
}

impl Ignore {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }
}

impl TextFilter for Ignore {
    fn name(&self) -> &'static str {
        &"ignore"
    }

    fn order(&self) -> u32 {
        HIGHEST_ORDER + 100
    }

    fn init(&mut self, config: &str) -> std::io::Result<()> {
        let mut cfg: IgnoreConfig =
            serde_json::from_str(config).map_err(|e| std::io::Error::other(e.to_string()))?;
        // ASC, speed up
        cfg.rules.sort();
        for r in cfg.rules.iter() {
            match Regex::new(r.as_str()) {
                Ok(r) => self.rules.push(r),
                Err(e) => {
                    log(format!("rule [{}] invalid [{}]", r, e));
                }
            }
        }
        Ok(())
    }

    fn filter(&mut self, _ctx: &mut Context, input: String) -> Option<String> {
        for rule in self.rules.iter() {
            if rule.is_match(input.as_str()) {
                return None;
            }
        }
        Some(input)
    }
}
