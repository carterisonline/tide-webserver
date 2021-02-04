use crate::Console;
use once_cell::sync::Lazy;
use std::env;
use std::process::Command;
use execute::Execute;
use colored::Colorize;

pub static WORKDIR: Lazy<String> = Lazy::new(|| {
    let out = env::var("WORKDIR").expect("Please provide a WORKDIR");
    if out.ends_with('/') {
        out
    } else {
        format!("{}/", out)
    }
});

pub static CONSOLE: Lazy<Console> = Lazy::new(|| Console::new());

pub static INDEX: Lazy<String> = Lazy::new(|| {
    CONSOLE.log(format!("{}", "Building index.html...".yellow()), true);

    let mut command = Command::new("parcel");
    command.arg("build");
    command.arg(format!("{}web/index.html", WORKDIR.as_str()));

    if let Some(exit_code) = command.execute().unwrap() {
        if exit_code == 0 {
            CONSOLE.log(format!("{}", "Successfully built index.html!".green()), true);
        } else {
            eprintln!("Failed to build index.html");
        }
    } else {
        eprintln!("Build process interrupted");
    }
    
    std::fs::read_to_string(format!("{}dist/index.html", WORKDIR.as_str()))
        .expect("Couldn\'t load `index.html`")
});

pub static ADDR: &'static str = "127.0.0.1:3000";