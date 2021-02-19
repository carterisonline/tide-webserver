use std::{
    io::{self, stdin, Write},
    process,
};

use crate::preloader::ADDR;
use crate::compiler::npm_install;
use colored::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;


pub static VERBOSE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn log(text: String, verbose: bool) {
    if !verbose {
        println!("{}", text);
    } else if verbose && *VERBOSE.lock().unwrap() {
        println!("{}: {}", "[VERBOSE LOGGER]".blue(), text);
    }
}

pub fn spawn() {
    npm_install().unwrap();
    std::thread::spawn(move || loop {
        print!(
            "[{} @{}] => ",
            "TIDE-WEBSERVER".yellow(),
            ADDR.green().italic()
        );
        io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        match line.trim_end_matches('\n').to_ascii_lowercase().as_str() {
            "exit" => process::exit(0x0100),
            "verbose" => {
                println!("{}", "Expected option <on>, <off>, or <toggle>".red());
            }
            "verbose on" => {
                *VERBOSE.lock().unwrap() = true;
                println!("{} {}", "Verbose logging is now".blue(), " ON ".black().on_green());
            },
            "verbose off" => {
                *VERBOSE.lock().unwrap() = false;
                println!("{} {}", "Verbose logging is now".blue(), " OFF ".black().on_red());
            },
            "verbose toggle" => {
                let mut b = VERBOSE.lock().unwrap();
                *b = !*b;
                match *b {
                    true => println!("{} {}", "Verbose logging is now".blue(), " ON ".black().on_green()),
                    false => println!("{} {}", "Verbose logging is now".blue(), " OFF ".black().on_red())
                }
            }
            "" => (),
            _ => println!(
                "{}",
                format!(
                    "\"{}\" is not a recognized command.",
                    line.trim_end_matches('\n')
                )
                .red()
            ),
        }
    });
}
