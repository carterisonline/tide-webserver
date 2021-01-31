use actix_web::{get, http::StatusCode, App, HttpResponse, HttpServer};

mod preloader;
use colored::Colorize;
use preloader::{INDEX, WORKDIR, ADDR};

mod ssl;
mod console;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(INDEX.as_str())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let builder = ssl::build(WORKDIR.as_str());

    let console = console::Console::spawn();
    console.log("Hello, World!".white(), false);

    HttpServer::new(|| App::new().service(index))
        .bind_openssl(ADDR, builder)?
        .run()
        .await
}
