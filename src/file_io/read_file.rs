use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Error;

pub struct FileReader;

impl FileReader {
    pub fn read_file(&self, file_path: &Path) -> Result<String, Error> {

        let mut file = match File::open(file_path) {
            Err(e) => return Err(e),
            Ok(file) => file,
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Err(e) => Err(e),
            Ok(_) => Ok(s),
        }
    }
}
