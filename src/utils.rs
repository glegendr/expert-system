
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

fn print_false_no_rule(mut s: String, query: char, variables: &HashMap<char, Variable>) {
    let value: char = s.pop().unwrap();
    if let Some(var) = variables.get(&value) {
        match var.value {
            true => {
                match &var.alias_true {
                    Some(alias) => println!("We know {} because no rule assign it.", alias.green()),
                    _ => println!("We know {} is {} because no rule assign it.", value.to_string().purple().bold(), "true".green()),
                }
            },
            false => {
                match &var.alias_false {
                    Some(alias) => println!("We know {} because no rule assign it.", alias.red()),
                    _ => println!("We know {} is {} because no rule assign it.", value.to_string().purple().bold(), "false".red()),
                }
            },
        }
    }
}

fn print_rules_path(s: String, variables: &HashMap<char, Variable>, query: char) {
    let formula = s.trim_start_matches('r').chars().fold((String::new(), false), |(mut acc, is_neg), c| {
        match c {
            '!' => {
                acc.pop();
                return (acc, true)
            },
            _ => {
                if c.is_alphabetic() {
                    if let Some(var) = variables.get(&c) {
                        let name = match is_neg {
                            true => var.alias_false.clone().unwrap_or(format!("!{c}")),
                            false => var.alias_true.clone().unwrap_or(c.to_string()),
                        };
                        return (format!("{acc}{name}"), false);
                    }
                }
                return (format!("{acc}{c}"), is_neg);
            }
        }
    }).0;
    print!("We have{}. ", formula.blue().bold());
    let mut implies_bool = false;
    let mut conjuction_word = "and";
    for c in s.chars() {
        if c == '=' {
            implies_bool = true;
        }
        if c == '>' && implies_bool == true {
            conjuction_word = "so"
        }
        if c != '=' {
            implies_bool = false;
        }
        if c.is_alphabetic() == true {
            if let Some(var) = variables.get(&c) {
                let name = match c == query {
                    true => c.to_string().purple().bold(),
                    false => c.to_string().yellow().bold()
                };
                match var.value {
                    true => {
                        match &var.alias_true {
                            Some(alias) => print!("{conjuction_word} {} ", alias.green()),
                            _ => print!("{conjuction_word} {name} is {} ", "true".green()),
                        }
                    },
                    false => {
                        match &var.alias_false {
                            Some(alias) => print!("{conjuction_word} {} ", alias.red()),
                            _ => print!("{conjuction_word} {name} is {} ", "false".red()),
                        }
                    },
                };
            }
        }
    }
    print!("\n");
}

fn print_already_know(mut s: String, query: char, variables: &HashMap<char, Variable>) {
    let value: char = s.pop().unwrap();
    if let Some(var) = variables.get(&value) {
        match var.value {
            true => {
                match &var.alias_true {
                    Some(alias) => println!("We already know that {}", alias.green()),
                    _ => println!("We already know that {} is {}", value.to_string().purple().bold(), "true".green()),
                }
            },
            false => {
                match &var.alias_false {
                    Some(alias) => println!("We already know that {}", alias.red()),
                    _ => println!("We already know that {} is {}", value.to_string().purple().bold(), "false".red()),
                }
            },
        }
    }
}

pub fn print_history(history: String, variables: &HashMap<char, Variable>, query: char) {
    let paths: Vec<&str> = history.split('%').collect();
    if let Some(var) = variables.get(&query) {
        match var.value {
            true => {
                match &var.alias_true {
                    Some(alias) => println!("we known {} because", alias.green()),
                    _ => println!("we known {} is {} because", query.to_string().purple().bold(), "true".green()),
                }
            },
            false => {
                match &var.alias_false {
                    Some(alias) => println!("we known {} because", alias.red()),
                    _ => println!("we known {} is {} because", query.to_string().purple().bold(), "false".red()),
                }
            },
        }
    }
    for path in paths.iter() {
        if path.len() > 0 {
            match path.chars().next().unwrap() {
                'r' => print_rules_path(path.to_string(), variables, query),
                'n' => print_false_no_rule(path.to_string(), query, variables),
                'i' => print_already_know(path.to_string(), query, variables),
                _ => unreachable!()
            }
        }
    }
}