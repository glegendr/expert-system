use std::env::args;
use crate::utils::string_to_char;
use crate::translate::Lang;

#[derive(PartialEq, Clone)]
pub enum Flag {
    Help,
    Interactive,
    Trace,
    Variables,
    Lang(Lang)
}

impl Flag {
    fn from_string(string: &str, iter: &mut std::slice::Iter<String>) -> Result<Self, String> {
        Ok(match string {
            "-h" | "--help" => Flag::Help,
            "-i" | "--interactive" => Flag::Interactive,
            "-t" | "--trace" => Flag::Trace,
            "-v" | "--variables" => Flag::Variables,
            "-l" | "--langage" => {
                match iter.next() {
                    Some(language) => {
                        match language.as_str().to_lowercase().trim() {
                            "en" => Flag::Lang(Lang::En),
                            "fr" => Flag::Lang(Lang::Fr),
                            "it" => Flag::Lang(Lang::It),
                            _ => Err(format!("{language} is unknown, try [en, fr, it]"))?
                        }
                    },
                    None => Err(format!("{string} need a language, try [en, fr, it]"))?
                }
            }
            _ => Err(format!("{string} is an undefined flag"))?
        })
    }
}

pub fn print_helper(error: &str) -> Result<(), String> {
    println!("cargo run --release -- [maps] [flags]");
    println!("-h, --help                     print this helper");
    println!("-i, --interactive              launch interactive mode");
    println!("-t, --trace                    display algorithm's trace");
    println!("-v, --variables                display variables before running algorithm (no interactive)");
    println!("-l, --langage  [fr, en, it]    change default language to chosen one");
    Err(String::from(error))
}

pub fn leaks() -> Result<(Vec<String>, Vec<Flag>), String> {
    let mut files: Vec<String> = Vec::default();
    let mut flags: Vec<Flag> = Vec::default();
    let args = args().collect::<Vec<String>>();
    let mut args_iter = args.iter();
    args_iter.next();
    loop {
        match args_iter.next() {
            Some(argument) => {
                match string_to_char(&argument) {
                    '-' => {
                        match Flag::from_string(&argument, &mut args_iter)? {
                            Flag::Help => print_helper("Help asked")?,
                            f => flags.push(f),
                        }
                    },
                    _ => files.push(String::from(argument)),
                }

            },
            None => break
        }
    }
    if files.len() == 0 && !flags.iter().any(|f| f == &Flag::Interactive) {
        print_helper("No map given")?
    }
    Ok((files, flags))
}
