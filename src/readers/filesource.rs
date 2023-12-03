use crate::readers::TextReader;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

pub struct FileSource {
    reader: BufReader<File>,
}

impl FileSource {
    pub fn new(filename: &str) -> std::io::Result<Self> {
        Ok(Self {
            reader: BufReader::new(File::open(filename)?),
        })
    }
}

impl TextReader for FileSource {
    fn read_line(&mut self) -> Option<String> {
        let mut s = String::new();
        match self.reader.read_line(&mut s) {
            Ok(size) => {
                if size > 0 {
                    return Some(s);
                } else {
                    sleep(Duration::from_secs(1));
                }
            }
            Err(e) => {
                println!("reader failed: {}", e);
            }
        }
        None
    }
}
