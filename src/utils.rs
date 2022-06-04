
use std::collections::HashMap;
use crate::models::Variable;
use colored::Colorize;

pub fn string_to_char(string: &str) -> char {
    string.chars().next().unwrap_or('/')
}

pub fn print_variables(variables: &HashMap<char, Variable>)  {

    let is_ok = |v| if v {
        "✓".green()
    } else {
        "x".red()
    };

    for (k, v) in variables.iter() {
        println!("{k}: [{}] [{}] [{}] {} {}",
            is_ok(v.value),
            is_ok(v.locked),
            is_ok(v.requested),
            v.alias_true.as_ref().unwrap_or(&String::default()),
            v.alias_false.as_ref().unwrap_or(&String::default())
        );
        for rule in &v.rules {
            println!(" - {rule}");
        }
    }
}