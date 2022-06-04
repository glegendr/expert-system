use std::env::args;
use crate::utils::string_to_char;

enum Flag {
    Help
}

impl Flag {
    fn from_string(string: &str) -> Result<Self, String> {
        Ok(match string {
            "-h" | "--help" => Flag::Help,
            _ => Err(format!("{string} is an undefined flag"))?
        })
    }
}

pub fn print_helper(error: &str) -> Result<(), String> {
    println!("cargo run --release -- [maps] [flags]");
    println!("-h, --help             print this helper");
    Err(String::from(error))
}

pub fn leaks() -> Result<Vec<String>, String> {
    let mut files: Vec<String> = Vec::default();
    let args = args().collect::<Vec<String>>();
    let mut args_iter = args.iter();
    args_iter.next();
    for argument in args_iter {
        match string_to_char(&argument) {
            '-' => {
                match Flag::from_string(&argument)? {
                    Flag::Help => print_helper("Help asked")?
                }
            },
            _ => files.push(String::from(argument)),
        }
    }
    if files.len() == 0 {
        print_helper("No map given")?
    }
    Ok(files)
}