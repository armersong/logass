#![allow(dead_code)]

use std::time::Duration;
use crate::readers::filesource::FileSource;
use crate::readers::TextReader;

mod plugins;
mod readers;
mod output;

fn main() -> std::io::Result<()>{
    let version = env!("CARGO_PKG_VERSION");
    let args:Vec<String> = std::env::args().collect();
    let prog_name = args[0].as_str();
    println!("{} version: {}", prog_name, version);
    let mut input = if args.len() > 1 {
        Box::new(FileSource::new(args[1].as_str())?)
    } else {
        let r:Box<dyn TextReader>  = Box::new(readers::build_reader());
        r
    };
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
