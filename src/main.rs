#![allow(dead_code)]

use crate::log::log;
use crate::readers::filesource::FileSource;
use crate::readers::TextReader;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::time::Duration;

pub mod log;
mod output;
mod plugins;
mod readers;

fn get_plugin_dir(name: &str) -> String {
    let plugin_dir = Path::new(name);
    if plugin_dir.is_dir() {
        plugin_dir.to_str().unwrap().to_string()
    } else {
        let exe_path = env::current_exe().expect("get exe path failed");
        let mut exe_path: Vec<&OsStr> = exe_path.iter().collect();
        exe_path.remove(exe_path.len() - 1);
        exe_path.push(OsStr::new(name));
        let plugin_path = exe_path.join(OsStr::new(std::path::MAIN_SEPARATOR_STR));
        plugin_path.to_str().unwrap().to_string()
    }
}

fn main() -> std::io::Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().collect();
    let prog_name = args[0].as_str();
    log(format!("{} version: {}", prog_name, version));
    let mut input = if args.len() > 1 {
        Box::new(FileSource::new(args[1].as_str())?)
    } else {
        let r: Box<dyn TextReader> = Box::new(readers::build_reader());
        r
    };
    let mut output = output::Output::new();
    let mut proc = plugins::processor::Processor::new();
    let plugin_dir = get_plugin_dir("plugins");
    proc.init(plugin_dir.as_str())?;
    log(format!(
        "\n\n\n========================================\n\n"
    ));
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
