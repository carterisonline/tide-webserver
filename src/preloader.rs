use once_cell::sync::Lazy;
use std::env;
use crate::Console;

pub static WORKDIR: Lazy<String> = Lazy::new(|| {
    let out = env::var("WORKDIR").expect("Please provide a WORKDIR");
    if out.ends_with('/') {
        out
    } else {
        format!("{}/", out)
    }
});

pub static INDEX: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string(format!("{}web/index.html", WORKDIR.as_str()))
        .expect("Couldn\'t load `index.html`")
});

pub static CONSOLE: Lazy<Console> = Lazy::new(|| {
    Console::new()
});

pub static ADDR: &'static str = "127.0.0.0:3000";