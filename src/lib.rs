use std::env;
use std::error::Error;
use std::fs;
#[derive(Debug)]
pub struct Files {
    pub names: Vec<String>,
    pub count: usize,
}

impl Files {
    pub fn new(args: &[String]) -> Result<Files, &str> {
        if args.len() < 2 {
            return Err("Incorrect number of arguments. Expected at least 1.")
        }
        let filenames: Vec<String> = args[1 .. args.len()].to_vec();
        let count = filenames.len();
        return Ok(Files { names: filenames, count });
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

pub fn multi_run(files: &Files, flags: &Flags) -> i32 {
    let mut exit_code: i32 = 0;
    let mut failed_files: usize = 0;
    for filename in &files.names {
        println!("\nProcessing file: {}", filename);
        if let Err(e) = run(&filename, &flags) {
            eprintln!("Error: {}", e);
            exit_code = 1;
            failed_files += 1;
        }
    }
    println!("Processed successfully: {}, failed {}", files.count - failed_files, failed_files);
    return exit_code;
}

pub fn run(filename: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(&filename)?;

    if !flags.enumerate_contents {
        println!("{} contains {} lines", filename, count(&file_contents));
    } else {
        let (count, enumerated_contents): (usize, String) = count_and_print(&file_contents);
        println!("{} contains {} lines", filename, count);
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
    fn test_get_single_line_count() {
        let contents = "\
There are
this many lines
here.";

        assert_eq!(3, count(contents));
    }

    #[test]
    fn test_get_single_line_count_and_contents() {
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

    #[test]
    fn test_parsing_multiple_args_success() {
        let args: &[String] = &["[/bin/target]".to_string(), "file1.txt".to_string(), "file2.txt".to_string(), "file3".to_string()];

        let files: Files = Files::new(args).unwrap();
        assert_eq!(3, files.names.len());
        assert_eq!("file1.txt", files.names[0]);
        assert_eq!("file2.txt", files.names[1]);
        assert_eq!("file3", files.names[2]);
    }

    #[test]
    fn test_parsing_multiple_args_failure() {
        let args: &[String] = &["[/bin/target]".to_string()];

        let err: &str = Files::new(args).unwrap_err();
        assert_eq!("Incorrect number of arguments. Expected at least 1.", err)
    }

    #[test]
    fn test_run_success() {
        let filename: &str = "test1.txt";
        let flags: Flags = Flags::new();
        assert_eq!(true, run(&filename, &flags).is_ok());
    }

    #[test]
    fn test_run_failure() {
        let filename: &str = "does-not-exist.abc";
        let flags: Flags = Flags::new();
        assert_eq!(true, run(&filename, &flags).is_err());
    }

    #[test]
    fn test_multi_run_success() {
        let filenames: Vec<String> = ["test1.txt".to_string(), "test2.txt".to_string()].to_vec();
        let count = filenames.len();
        let files: Files = Files { names: filenames, count };
        let flags: Flags = Flags::new();
        assert_eq!(0, multi_run(&files, &flags));
    }

    #[test]
    fn test_multi_run_failure() {
        let filenames: Vec<String> = ["test2.txt".to_string(), "does-not-exist.abc".to_string()].to_vec();
        let count = filenames.len();
        let files: Files = Files { names: filenames, count };
        let flags: Flags = Flags::new();
        assert_eq!(1, multi_run(&files, &flags))
    }
}