use std::{
    io::{self, stdin, Write},
    process,
};

use crate::preloader::ADDR;
use colored::*;

pub struct Console {
    verbose: bool,
}

impl Console {
    pub fn log(self, text: ColoredString, verbose: bool) {
        if (verbose && self.verbose) || !verbose {
            println!("{}", text);
        }
    }

    pub fn new() -> Console {
        Console { verbose: false }
    }

    pub async fn spawn(&mut self) {
        crossbeam::scope(|scope| {
            scope.spawn(move |_| loop {
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
                        self.verbose = !&self.verbose;
                    }
                    "log-verbose" => {
                        if self.verbose {
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
        })
        .expect("A child thread panicked");
    }
}
