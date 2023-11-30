use std::time::Duration;
use crate::readers::TextReader;

mod plugins;
mod readers;
mod output;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let prog_name = std::env::args().nth(0).unwrap();
    println!("{} version: {}", prog_name, version);
    let mut input = readers::build_reader();
    let mut output = output::Output::new();
    let mut proc = plugins::processor::Processor::new();
    proc.init("plugins").expect("processor init failed");
    loop {
        if let Some(data) = input.read_line() {
            if let Some(out_data) = proc.handle(data) {
                output.output(out_data);
            }
        } else {
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
