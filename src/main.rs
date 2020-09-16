use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, error};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
struct TranslateRequest {
    phrase: String,
    //target_language: String
}

async fn help(req: HttpRequest) -> impl Responder {
    let path = Path::new("./help.txt");
    
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open {}: {}", path.display(), e),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(e) => panic!("Couldn't read{}: {}", path.display(), e),
        Ok(_) => format!("{}", s),
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
