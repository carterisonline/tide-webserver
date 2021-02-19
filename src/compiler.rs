use crate::console;
use crate::preloader::WORKDIR;
use colored::Colorize;
use execute::Execute;
use std::process::{self, Command};

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
            println!("{}", "Couldn\'t fetch NPM packages! Is Node.js installed?".red());
            println!("For Arch/Manjaro: {}", "pacman -S nodejs npm".blue());
            println!("For RHEL-Based Distros: {}", "dnf module install nodejs:latest".magenta());
            println!("For Debian/Ubuntu: ------------------");
            println!("{}", "curl -fsSL https://deb.nodesource.com/setup_15.x | sudo -E bash -".red());
            println!("{}", "sudo apt-get install -y nodejs".red());
            process::exit(0x0100);
        }
    } else {
        Err(String::from("NPM fetch process interrupted"))
    }
}