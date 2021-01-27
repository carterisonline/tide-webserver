use actix_web::{get, http::StatusCode, App, HttpResponse, HttpServer};

mod preloader;
use preloader::{INDEX, WORKDIR};

mod ssl;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(INDEX.as_str())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let builder = ssl::build(WORKDIR.as_str());

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:12787", builder)?
        .run()
        .await
}
