use std::{
    io::{self, stdin, Write},
    process, thread,
};

use crate::preloader::ADDR;
use colored::*;
use thread::JoinHandle;

pub struct Console<T> {
    verbose: bool,
    thread: JoinHandle<T>,
}

impl Console<()> {
    pub fn log(self: Self, text: ColoredString, verbose: bool) {
        if (verbose && self.verbose) || !verbose {
            println!("{}", text);
        }
    }

    pub fn spawn() -> Console<()> {
        Console {
            verbose: false,
            thread: thread::spawn(move || loop {
                print!(
                    "[{} @{}] => ",
                    "ACTIX-WEBSERVER".yellow(),
                    ADDR.green().italic()
                );
                io::stdout().flush().unwrap();
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                handle_line(line);
            }),
        }
    }
}

fn handle_line(line: String) {
    match line.trim_end_matches('\n').to_ascii_lowercase().as_str() {
        "exit" => process::exit(0x0100),
        "toggle-verbose" | "verbose" => {}
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
}
