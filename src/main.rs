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

    let exit_code: i32 = simple_line_counter::multi_run(&files, &flags);
    process::exit(exit_code);
}

