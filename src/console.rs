use std::{
    io::{self, stdin, Write},
    process,
};

use crate::preloader::ADDR;
use colored::*;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::io::Read;

pub struct Console {}

static LOGS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
pub static VERBOSE: Lazy<Mutex<AtomicBool>> = Lazy::new(|| Mutex::new(AtomicBool::new(false)));

impl Console {
    pub fn log(&mut self, text: String, verbose: bool) {
        if !verbose {
            LOGS.lock().unwrap().push(text);
            // println!("{}", text);
        } else if verbose && VERBOSE.lock().unwrap().load(Ordering::SeqCst) {
            LOGS.lock().unwrap().push(format!("{}: {}", "[VERBOSE LOGGER]".blue(), text));
            // println!("{}: {}", "[VERBOSE LOGGER]".blue(), text);
        }
    }

    pub fn new() -> Console {
        Console {}
    }

    pub fn spawn(&self) {
        std::thread::spawn(move || loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            print!(
                "[{} @{}] => ",
                "ACTIX-WEBSERVER".yellow(),
                ADDR.green().italic()
            );

            let l = LOGS.lock().unwrap();
            for i in 0..1 {
                println!("{}", l.get(i).unwrap_or(&String::from("")))
            }

            let mut input = String::from("");
            io::stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();

            println!("{}", LOGS.lock().unwrap().len());

            match input.trim_end_matches('\n').to_ascii_lowercase().as_str() {
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
                        input.trim_end_matches('\n')
                    )
                    .red()
                ),
            }
        });
    }
}
