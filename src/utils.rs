
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

fn print_false_no_rule(mut s: String, query: char) {
    let value: char = s.pop().unwrap();
    if value == query {
        println!("We know {} is {} because no rule assign it.", value.to_string().purple().bold(), "false".red());
    } else {
        println!("We know {} is {} because no rule assign it.", value.to_string().yellow().bold(), "false".red());
    }
}

fn print_rules_path(s: String, variables: &HashMap<char, Variable>, query: char) {
    print!("We have{}. ", s.trim_start_matches('r').blue().bold());
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
                let value = match var.value {
                    true => var.value.to_string().green(),
                    _ => var.value.to_string().red()
                };
                let name = match c == query {
                    true => c.to_string().purple().bold(),
                    false => c.to_string().yellow().bold()
                };
                print!("{conjuction_word} we known {name} is {value} ");
            }
        }
    }
    print!("\n");
}

fn print_already_know(mut s: String, query: char) {
    let value: char = s.pop().unwrap();
    if value == query {
        println!("We already know that {} is {}.", value.to_string().purple().bold(), "true".green());
    } else {
        println!("We already know that {} is {}.", value.to_string().yellow().bold(), "true".green());
    }
}

pub fn print_history(history: String, variables: &HashMap<char, Variable>, query: char) {
    let paths: Vec<&str> = history.split('%').collect();
    if let Some(var) = variables.get(&query) {
        if var.value == true {
            println!("we know {} is {} because", query.to_string().purple().bold(), var.value.to_string().green());
        } else {
            println!("we know {} is {} because", query.to_string().purple().bold(), var.value.to_string().red());
        }
    }
    for path in paths.iter() {
        if path.len() > 0 {
            match path.chars().next().unwrap() {
                'r' => print_rules_path(path.to_string(), variables, query),
                'n' => print_false_no_rule(path.to_string(), query),
                'i' => print_already_know(path.to_string(), query),
                _ => unreachable!()
            }
        }
    }
}