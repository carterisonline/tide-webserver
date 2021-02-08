use crate::Console;
use colored::Colorize;
use execute::Execute;
use once_cell::sync::Lazy;
use std::env;
use std::process::Command;
use std::sync::{Mutex};

pub static WORKDIR: Lazy<String> = Lazy::new(|| {
    let out = env::var("WORKDIR").unwrap_or(
        env::var("PWD").expect("Please provide a WORKDIR"),
    );
    if out.ends_with('/') {
        out
    } else {
        format!("{}/", out)
    }
});

pub static SSL: Lazy<bool> = Lazy::new(|| {
    let out = env::var("SSL").unwrap_or(String::from("false"));
    if out == "true" {
        true
    } else if out == "false" {
        false
    } else {
        eprintln!(
            "\"{}\" {}",
            out.blue(),
            "is not a valid option for SSL; expected \"true\" or \"false\"".red()
        );
        panic!("Unexpected token for environment variable \"SSL\"");
    }
});

pub static CONSOLE: Lazy<Mutex<Console>> = Lazy::new(|| Mutex::new(Console::new()));

pub static INDEX: Lazy<String> = Lazy::new(|| {
    CONSOLE.lock().unwrap().log(format!("{}", "Building index.pug...".yellow()), true);

    let mut command = Command::new("parcel");
    command.arg("build");
    command.arg(format!("{}web/index.pug", WORKDIR.as_str()));

    if let Some(exit_code) = command.execute().unwrap() {
        if exit_code == 0 {
            CONSOLE.lock().unwrap().log(
                format!("{}", "Successfully built index.pug!".green()),
                true,
            );
        } else {
            eprintln!("Failed to build index.html");
        }
    } else {
        eprintln!("Build process interrupted");
    }
    std::fs::read_to_string(format!("{}dist/index.html", WORKDIR.as_str()))
        .expect("Couldn\'t load `index.html`")
});

pub static ADDR: &'static str = "0.0.0.0:12787";
