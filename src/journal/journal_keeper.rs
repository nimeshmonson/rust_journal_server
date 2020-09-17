use crate::file_io::read_file::FileReader;
use crate::file_io::write_file::FileWriter;
use super::journal_request::{JournalRetrieveRequest, JournalEntryRequest};
use std::io::Error;
use std::path::Path;

pub struct JournalKeeper;


impl JournalKeeper{
    //retrieves journal entries
    pub fn retrieve(&self, path: &Path) -> Result<String, Error> {
        FileReader.read_file(Path::new(&path)) 
    }

    //Enters phrases to the journal with timestamp
    pub fn enter(&self, path: &Path, phrase: &String, time: String) -> Result<String, Error> {
        match path.exists() {
            false => {
                return FileWriter.create_file(Path::new(&path), phrase.to_string(), time);
            },
            true => {
                return FileWriter.append_file(Path::new(&path), phrase.to_string(), time);
            }
        }
    }
}
