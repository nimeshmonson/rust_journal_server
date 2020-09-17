use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::io::Error;

pub struct FileWriter;

impl FileWriter {
    //creates and writes to file if it doesnt exist
    pub fn create_file(&self, file_path: &Path, phrase: String, time :String) -> Result<String, Error> {

        let mut file = match File::create(file_path) {
            Err(e) => return Err(e),
            Ok(file) => file,
        };

        match file.write_all(String::from(format!("{} : {}\n", time, phrase)).as_str().as_bytes()){
            Err(e) => Err(e),
            Ok(_) => Ok(String::from(format!("Successfully created {}", file_path.display())))
        }
    }

    //appends to an existing file
    pub fn append_file(&self, file_path: &Path, phrase: String, time :String) -> Result<String, Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();

        match file.write_all(String::from(format!("{} : {}\n", time, phrase)).as_str().as_bytes()){
            Err(e) => Err(e),
            Ok(_) => Ok(String::from(format!("Successfully appended to file {}", file_path.display())))
        }
    }
}
