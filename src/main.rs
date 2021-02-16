use tide::{http::mime, Request, Response, Result, StatusCode};
use tide_rustls::TlsListener;

mod preloader;
use colored::Colorize;
use preloader::{ADDR, INDEX, WORKDIR, SSL};

mod console;

async fn index(req: Request<()>) -> Result<Response> {
    console::log(
        format!(
            "[{}]: {} {}",
            req.local_addr().unwrap_or("UNKNOWN IP").yellow(),
            req.method().to_string().green(),
            req.url()
                .to_string()
                .trim_start_matches(
                    format!("{}://{}", &match *SSL {
                        true => "https",
                        false => "http",
                    }, ADDR).as_str()
                )
                .red()
        ),
        true
    );

    Ok(Response::builder(StatusCode::Ok)
        .body(INDEX.as_str())
        .content_type(mime::HTML)
        .build())
}

#[async_std::main]
async fn main() -> Result<()> {
    console::spawn();

    let mut app = tide::new();
    app.at("/").get(index);

    if *SSL {
        app.listen(
            TlsListener::build()
                .addrs(ADDR)
                .cert(format!("{}keys/cert.pem", *WORKDIR))
                .key(format!("{}keys/key.pem", *WORKDIR)),
        ).await?;
    } else {
        app.listen(
            ADDR
        ).await?;
    }
    
    Ok(())
}
