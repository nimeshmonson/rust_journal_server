use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct JournalEntryRequest {
    pub phrase: String,
}

impl fmt::Display for JournalEntryRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "phrase: {}", self.phrase)
    }
}

#[derive(Deserialize, Debug)]
pub struct JournalRetrieveRequest {
    pub year: String,
    pub month: String,
    pub date: String
}

impl fmt::Display for JournalRetrieveRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date: {}, month: {}, year: {}", self.date, self.month, self.year)
    }
}
