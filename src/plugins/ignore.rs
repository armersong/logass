use crate::plugins::{HIGHEST_ORDER, TextFilter};

pub struct Ignore {

}

impl TextFilter for Ignore {
    fn name(&self) -> &'static str {
        &"ignore"
    }

    fn order(&self) -> u32 {
        HIGHEST_ORDER
    }

    fn init(&mut self, config: &str) -> std::io::Result<()> {
        todo!()
    }

    fn filter(&mut self, input: String) -> Option<String> {
        todo!()
    }
}