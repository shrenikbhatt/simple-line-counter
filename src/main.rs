use std::env;
use std::process;

use simple_line_counter::{ Files, Flags };

fn main() {
    let args: Vec<String> = env::args().collect();

    let flags: Flags = Flags::new();
    let files: Files = Files::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    for filename in files.names {
        println!("\nProcessing file: {}", filename);
        if let Err(e) = simple_line_counter::run(&filename, &flags) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

