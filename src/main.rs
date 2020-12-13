use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://stoic-booth-3c5113.netlify.app/")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
