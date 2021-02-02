use actix_web::{get, http::StatusCode, App, HttpRequest, HttpResponse, HttpServer};

mod preloader;
use colored::Colorize;
use console::Console;
use preloader::{ADDR, CONSOLE, INDEX, WORKDIR};

mod console;
mod ssl;

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    CONSOLE.log(
        format!(
            "[{}]: {} {}",
            req.connection_info()
                .realip_remote_addr()
                .unwrap_or("UNKNOWN IP")
                .yellow(),
            req.method().to_string().green(),
            req.path().red()
        ),
        true,
    );
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(INDEX.as_str())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let builder = ssl::build(WORKDIR.as_str());

    CONSOLE.spawn();
    CONSOLE.log("Hello, World!".white(), false);

    HttpServer::new(|| App::new().service(index))
        .bind_openssl(ADDR, builder)?
        .run()
        .await
}
