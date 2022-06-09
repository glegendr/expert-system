use std::env::args;
use crate::utils::string_to_char;

#[derive(PartialEq)]
pub enum Flag {
    Help,
    Interactive,
    Trace
}

impl Flag {
    fn from_string(string: &str) -> Result<Self, String> {
        Ok(match string {
            "-h" | "--help" => Flag::Help,
            "-i" | "--interactive" => Flag::Interactive,
            "-t" | "--trace" => Flag::Trace,
            _ => Err(format!("{string} is an undefined flag"))?
        })
    }
}

pub fn print_helper(error: &str) -> Result<(), String> {
    println!("cargo run --release -- [maps] [flags]");
    println!("-h, --help             print this helper");
    println!("-i, --interactive      launch interactive mode");
    Err(String::from(error))
}

pub fn leaks() -> Result<(Vec<String>, Vec<Flag>), String> {
    let mut files: Vec<String> = Vec::default();
    let mut flags: Vec<Flag> = Vec::default();
    let args = args().collect::<Vec<String>>();
    let mut args_iter = args.iter();
    args_iter.next();
    for argument in args_iter {
        match string_to_char(&argument) {
            '-' => {
                match Flag::from_string(&argument)? {
                    Flag::Help => print_helper("Help asked")?,
                    f => flags.push(f),
                }
            },
            _ => files.push(String::from(argument)),
        }
    }
    if files.len() == 0 && !flags.iter().any(|f| f == &Flag::Interactive) {
        print_helper("No map given")?
    }
    Ok((files, flags))
}
