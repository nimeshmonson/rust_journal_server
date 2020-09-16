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

async fn help(req: HttpRequest) -> impl Responder {

    match FileReader.read_file(Path::new("./help.txt")) {
        Err(e) => format!("Error opening help.txt: {}", e),
        Ok(s) => s
    }
}

async fn index(req: HttpRequest) -> impl Responder {
    format!("Index Method")
}

async fn ping(req: HttpRequest) -> impl Responder {
    format!("Pong")
}

async fn translate(req: web::Json<TranslateRequest>) -> impl Responder {
    format!("translate")
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
