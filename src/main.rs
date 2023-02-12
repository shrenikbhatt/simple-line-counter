use std::env;
use std::process;

use simple_line_counter::{ File, Flags };

fn main() {
    let args: Vec<String> = env::args().collect();

    let flags: Flags = Flags::new();
    let file: File = File::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    println!("Processing file: {}", file.name);

    if let Err(e) = simple_line_counter::run(&file, &flags) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

