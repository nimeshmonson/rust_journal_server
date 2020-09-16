use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, error};

#[derive(Deserialize, Debug)]
struct TranslateRequest {
    phrase: String,
    //target_language: String
}

async fn help(req: HttpRequest) -> impl Responder {
    format!("Help method")
    //TODO add a txt file with instructions and
    //return here
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
        App::new()
            .app_data(json_config)
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(ping))
            .route("/help", web::get().to(help))
            .route("/translate", web::post().to(translate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
