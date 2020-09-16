use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, error};
use serde::{Serialize, Deserialize};
use std::path::Path;

mod file_io;

use file_io::read_file::FileReader;

#[derive(Deserialize, Debug)]
struct TranslateRequest {
    phrase: String,
    //target_language: String
}

//Returns instructions on how to use the Translate API
async fn help(req: HttpRequest) -> HttpResponse {
    match FileReader.read_file(Path::new("./help.txt")) {
        Err(e) => HttpResponse::Ok().body((format!("Error opening help.txt: {}", e))),
        Ok(s) => HttpResponse::Ok().body(s)
    }
}

//Returns a welcome message with some API routes 
async fn index(req: HttpRequest) -> HttpResponse {
    match FileReader.read_file(Path::new("./index.txt")) {
        Err(e) => HttpResponse::Ok().body((format!("Error opening help.txt: {}", e))),
        Ok(s) => HttpResponse::Ok().body(s)
    }
}

//Route checks if the API is up and running
async fn ping(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

async fn translate(req: web::Json<TranslateRequest>) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {     
        App::new()
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(ping))
            .route("/help", web::get().to(help))
            .route("/translate", web::post().to(translate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
