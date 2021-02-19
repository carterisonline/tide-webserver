use std::{
    io::{self, stdin, Write},
    process,
};

use crate::preloader::{ADDR, npm_install};
use colored::*;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;


pub static VERBOSE: Lazy<Mutex<AtomicBool>> = Lazy::new(|| Mutex::new(AtomicBool::new(false)));

pub fn log(text: String, verbose: bool) {
    if !verbose {
        println!("{}", text);
    } else if verbose && VERBOSE.lock().unwrap().load(Ordering::SeqCst) {
        println!("{}: {}", "[VERBOSE LOGGER]".blue(), text);
    }
}

pub fn spawn() {
    npm_install().unwrap();
    std::thread::spawn(move || loop {
        print!(
            "[{} @{}] => ",
            "ACTIX-WEBSERVER".yellow(),
            ADDR.green().italic()
        );
        io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        match line.trim_end_matches('\n').to_ascii_lowercase().as_str() {
            "exit" => process::exit(0x0100),
            "verbose-on" => *VERBOSE.lock().unwrap().get_mut() = true,
            "verbose-off" => *VERBOSE.lock().unwrap().get_mut() = false,
            "log-verbose" => {
                if VERBOSE.lock().unwrap().load(Ordering::SeqCst) {
                    println!("{}", "Hello, Verbose!".yellow());
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
