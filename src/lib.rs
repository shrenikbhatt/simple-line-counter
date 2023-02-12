use std::env;
use std::error::Error;
use std::fs;

pub struct File {
    pub name: String,
}

impl File {
    pub fn new(args: &[String]) -> Result<File, &str> {
        if args.len() != 2 {
            return Err("Incorrect number of arguments. Expected 1.")
        }
        let filename: String = args[1].clone();
        return Ok(File { name: filename });
    }
}

pub struct Flags {
    pub enumerate_contents: bool,
}

impl Flags {
    pub fn new() -> Flags {
        let enumerate_contents = env::var("ENUMERATE_CONTENTS").is_ok();
        return Flags { enumerate_contents };
    }
}

pub fn run(file: &File, flags: &Flags) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(&file.name)?;

    if !flags.enumerate_contents {
        println!("{} contains {} lines", file.name, count(&file_contents));
    } else {
        let (count, enumerated_contents): (usize, String) = count_and_print(&file_contents);
        println!("{} contains {} lines", file.name, count);
        println!("{}", enumerated_contents);
    }
    
    return Ok(());
}

pub fn count(contents: &str) -> usize {
    return contents.lines().count();
}

pub fn count_and_print(contents: &str) -> (usize, String) {
    let mut i: usize = 1;
    let mut enumerated_contents: String = String::new();

    for line in contents.lines() {
        if i != 1 {
            enumerated_contents.push('\n');
        }
        enumerated_contents.push_str(&format!("{}. {}", i, line));
        i+=1;
    }
    return (i-1, enumerated_contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_line_count() {
        let contents = "\
There are
this many lines
here.";

        assert_eq!(3, count(contents));
    }

    #[test]
    fn get_line_count_and_contents() {
        let contents: &str = "\
There are
this many lines
in this one
test.";

        let result: String = "\
1. There are
2. this many lines
3. in this one
4. test.".to_string();

        assert_eq!((4, result), count_and_print(contents));
    }
}