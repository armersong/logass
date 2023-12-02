mod ignore;
mod color;
pub mod processor;
mod line_num;

const HIGHEST_ORDER:u32 = 0;
const LOWEST_ORDER:u32 = 1000;

pub struct Context {

}

pub trait TextFilter{
    fn name(&self) -> &'static str;

    /// lower is higher
    fn order(&self) -> u32;

    fn init(&mut self, config:&str) -> std::io::Result<()>;

    /// handle input, output
    /// @return
    fn filter(&mut self, ctx: &mut Context, input: String) -> Option<String>;
}