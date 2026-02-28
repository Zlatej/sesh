use std::process;

mod cache;
mod config;
mod scanner;
mod utils;

fn main() {
    let cfg = match config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            process::exit(1);
        }
    };
    println!("Successfully loaded config:\n{:#?}", cfg);
}
