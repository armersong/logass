use crate::plugins::{Context, TextFilter, LOWEST_ORDER};

pub struct LineNum {
    seq: u64,
}

impl LineNum {
    pub fn new() -> Self {
        Self { seq: 0 }
    }
}

impl TextFilter for LineNum {
    fn name(&self) -> &'static str {
        &"line"
    }

    fn order(&self) -> u32 {
        LOWEST_ORDER
    }

    fn init(&mut self, _config: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn filter(&mut self, _ctx: &mut Context, input: String) -> Option<String> {
        self.seq += 1;
        Some(format!("{:6}: {}", self.seq, input))
    }
}
