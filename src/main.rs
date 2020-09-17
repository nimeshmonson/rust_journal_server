use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, error};
use serde::{Serialize, Deserialize};
use std::path::Path;
use chrono::prelude::*;

mod file_io;
mod journal;

use file_io::read_file::FileReader;
use journal::journal_keeper::JournalKeeper;
use journal::journal_request::JournalRetrieveRequest;
use journal::journal_request::JournalEntryRequest;

//handler for GET to the Journal
async fn retrieve_journal(req: web::Query<JournalRetrieveRequest>) -> HttpResponse {
    
    let path = String::from(format!("./src/journal/files/{}-{}-{}.txt", req.month, req.date, req.year));

    match JournalKeeper.retrieve(Path::new(&path)) {
        Err(e) => HttpResponse::Ok().body(format!("Error retrieving journal: {}", e)),
        Ok(s) => HttpResponse::Ok().body(s)
    }
}

//handler for POST to the journal
async fn enter_journal(req: web::Query<JournalEntryRequest>) -> HttpResponse {

    let local : DateTime<Local> = Local::now();
    let date = local.day();
    let year = local.year();
    let month = local.month();
    let time = local.time().to_string();

    let path = String::from(format!("./src/journal/files/{}-{}-{}.txt", month, date, year));

    match JournalKeeper.enter(Path::new(&path), &req.phrase, time) {
        Err(e) => HttpResponse::Ok().body(format!("Error entering journal: {}", e)),
        Ok(s) => HttpResponse::Ok().body(s)
    }
}

//Returns a welcome message with some API routes 
async fn index(_req: HttpRequest) -> HttpResponse {
    match FileReader.read_file(Path::new("./index.txt")) {
        Err(e) => HttpResponse::Ok().body(format!("Error opening index.txt: {}", e)),
        Ok(s) => HttpResponse::Ok().body(s)
    }
}

//Route checks if the API is up and running
async fn ping(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {     
        App::new()
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(ping))
            .route("/journal", web::post().to(enter_journal))
            .route("/journal", web::get().to(retrieve_journal))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
