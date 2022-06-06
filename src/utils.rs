
use std::collections::HashMap;
use crate::models::Variable;
use colored::{Colorize, ColoredString};

pub fn string_to_char(string: &str) -> char {
    string.chars().next().unwrap_or('/')
}

pub fn tick_or_cross(b: bool) -> ColoredString {
    match b {
        true =>  "✓".green(),
        false => "x".red()
    }
}

pub fn print_variable(variable: (&char, &Variable)) {
    println!("{}: {}", variable.0, variable.1);
    for rule in &variable.1.rules {
        println!(" - {rule}");
    }
}

pub fn print_variables(variables: &HashMap<char, Variable>)  {
    for variable in variables {
        print_variable(variable);
    }
}

pub fn print_rules(variables: &HashMap<char, Variable>)  {
    let mut i = 0;
    for (_, v) in variables.iter() {
        for rule in &v.rules {
            i += 1;
            println!("[{i}] {rule}");
        }
    }
}