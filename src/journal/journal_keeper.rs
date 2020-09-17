use chrono::prelude::*;
use crate::file_io::read_file::FileReader;
use super::journal_request::{JournalRetrieveRequest, JournalEntryRequest};
use std::io::Error;
use std::path::Path;

pub struct JournalKeeper;


impl JournalKeeper{
    pub fn retrieve(&self, path: &Path) -> Result<String, Error> {
        FileReader.read_file(Path::new(&path)) 
    }

    pub fn enter(&self, phrase: &String ) -> Result<String, Error> {
        Ok(String::from(format!("{}", phrase)))
    }
}
