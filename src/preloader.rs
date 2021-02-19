use crate::console;
use colored::Colorize;
use execute::Execute;
use once_cell::sync::Lazy;
use std::env;
use std::process::Command;

fn compile_app(directory: &str) -> Result<(), String> {
    console::log(format!("{}", format!("Building {}...", directory).yellow()), true);

    let mut build_command = Command::new("npx");
    build_command.arg("parcel");
    build_command.arg("build");
    build_command.arg(format!("{}{}", WORKDIR.as_str(), directory));

    if let Some(exit_code) = build_command.execute().unwrap() {
        if exit_code == 0 {
            console::log(
                format!("{}", "Built successfully!".green()),
                true,
            );
            Ok(())
        } else {
            Err(format!("Failed to build {}", directory))
        }
    } else {
        Err(String::from("Build process interrupted"))
    }
}

pub fn npm_install() -> Result<(), String> {
    console::log(format!("{}", "Resolving NPM package configuration...".yellow()), true);
    let mut npm_command = Command::new("npm");
    npm_command.arg("install");
    if let Some(exit_code) = npm_command.execute().unwrap() {
        if exit_code == 0 {
            console::log(
                format!("{}", "All NPM packages installed.".green()),
                true,
            );
            Ok(())
        } else {
            Err(String::from("Failed to fetch NPM packages"))
        }
    } else {
        Err(String::from("NPM fetch process interrupted"))
    }
}

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
