use tide::{http::mime, Request, Response, Result, StatusCode};
use tide_rustls::TlsListener;

mod preloader;
use colored::Colorize;
use console::Console;
use preloader::{ADDR, CONSOLE, INDEX, WORKDIR};

mod console;

async fn index(req: Request<()>) -> Result<Response> {
    CONSOLE.log(
        format!(
            "[{}]: {} {}",
            req.local_addr().unwrap_or("UNKNOWN IP").yellow(),
            req.method().to_string().green(),
            req.url()
                .to_string()
                .trim_start_matches(
                    format!("https://{}", ADDR.replace("127.0.0.1", "localhost")).as_str()
                )
                .red()
        ),
        true,
    );

    Ok(Response::builder(StatusCode::Ok)
        .body(INDEX.as_str())
        .content_type(mime::HTML)
        .build())
}

#[async_std::main]
async fn main() -> Result<()> {
    println!("https://{}", ADDR);
    CONSOLE.spawn();
    CONSOLE.log(format!("{}", "Hello, World!".white()), false);
    let mut app = tide::new();
    app.at("/").get(index);
    app.listen(
        TlsListener::build()
            .addrs(ADDR)
            .cert(format!("{}keys/cert.pem", *WORKDIR))
            .key(format!("{}keys/key.pem", *WORKDIR)),
    )
    .await?;
    Ok(())
}
