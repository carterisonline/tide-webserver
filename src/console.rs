use std::{
    io::{self, stdin, Write},
    process,
};

use crate::preloader::ADDR;
use colored::*;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub struct Console {
    verbose: AtomicBool,
}

pub static VERBOSE: Lazy<Mutex<AtomicBool>> = Lazy::new(|| Mutex::new(AtomicBool::new(false)));

impl Console {
    pub fn log(self, text: ColoredString, verbose: bool) {
        if (verbose && self.verbose.into_inner()) || !verbose {
            println!("{}", text);
        }
    }

    pub fn new() -> Console {
        Console {
            verbose: AtomicBool::new(false),
        }
    }

    pub fn spawn(&self) {
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
                "toggle-verbose" | "verbose" => {
                    let v = &VERBOSE.lock().unwrap();
                    *VERBOSE.lock().unwrap().get_mut() = !v.load(Ordering::SeqCst);
                }
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
}
