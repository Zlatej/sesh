use std::process;

mod config;

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
