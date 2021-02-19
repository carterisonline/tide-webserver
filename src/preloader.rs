use crate::console;
use crate::compiler::compile_app;
use colored::Colorize;
use once_cell::sync::Lazy;
use std::env;

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

pub static INDEX: Lazy<String> = Lazy::new(|| {
    console::log(format!("{}", "Recieved first-time connection to Index! Preparing to build...".yellow()), true);

    compile_app("web/index.pug").unwrap();

    let out = std::fs::read_to_string(format!("{}dist/index.html", WORKDIR.as_str()));
    match out {
        Ok(_) => {
            console::log(format!("{}", "Loaded built index successfully!".green()), true);
            out.unwrap()
        }

        Err(err) => {
            eprintln!("Failed to load built index: {}", err);
            String::from("")
        }
    }
});

pub static ADDR: &'static str = "0.0.0.0:12787";
