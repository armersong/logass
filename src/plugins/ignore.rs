use crate::plugins::{Context, HIGHEST_ORDER, TextFilter};

pub struct Ignore {

}

impl TextFilter for Ignore {
    fn name(&self) -> &'static str {
        &"ignore"
    }

    fn order(&self) -> u32 {
        HIGHEST_ORDER
    }

    fn init(&mut self, _config: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn filter(&mut self, _ctx: &mut Context, input: String) -> Option<String> {
        Some(input)
    }
}