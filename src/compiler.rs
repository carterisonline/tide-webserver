use crate::console;
use crate::preloader::WORKDIR;
use colored::Colorize;
use execute::Execute;
use std::process::Command;

pub fn compile_app(directory: &str) -> Result<(), String> {
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