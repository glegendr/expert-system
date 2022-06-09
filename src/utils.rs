
use std::collections::HashMap;
use crate::models::Variable;
use colored::{Colorize, ColoredString};
use crate::translate::{Lang, Translate};

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

fn print_false_no_rule(mut s: String, variables: &HashMap<char, Variable>, lang: &Lang) {
    let value: char = s.pop().unwrap();
    if let Some(var) = variables.get(&value) {
        match var.value {
            true => {
                match &var.alias_true {
                    Some(alias) => Translate::NoRule.print(lang, alias.green(), None),
                    _ => Translate::NoRule.print(lang, value.to_string().purple().bold(), Some(var.value)),
                }
            },
            false => {
                match &var.alias_false {
                    Some(alias) => Translate::NoRule.print(lang, alias.red(), None),
                    _ => Translate::NoRule.print(lang, value.to_string().purple().bold(), Some(var.value)),
                }
            },
        }
    }
}

fn print_rules_path(s: String, variables: &HashMap<char, Variable>, query: char, lang: &Lang) {
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
    Translate::Rule.print(lang, formula.blue().bold(), None);
    let mut implies_bool = false;
    let mut conjuction_word = Translate::And;
    for c in s.chars() {
        if c == '=' {
            implies_bool = true;
        }
        if c == '>' && implies_bool == true {
            conjuction_word = Translate::So;
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
                            Some(alias) => conjuction_word.print(lang, alias.green(), None),
                            _ => conjuction_word.print(lang, name, Some(var.value)),
                        }
                    },
                    false => {
                        match &var.alias_false {
                            Some(alias) => conjuction_word.print(lang, alias.red(), None),
                            _ => conjuction_word.print(lang, name, Some(var.value)),
                        }
                    },
                };
            }
        }
    }
    print!("\n");
}

fn print_already_know(mut s: String, variables: &HashMap<char, Variable>, lang: &Lang) {
    let value: char = s.pop().unwrap();
    if let Some(var) = variables.get(&value) {
        match var.value {
            true => {
                match &var.alias_true {
                    Some(alias) => Translate::WeAlreadyKnow.print(lang, alias.green(), None),
                    _ => Translate::WeAlreadyKnow.print(lang, value.to_string().purple().bold(),Some(var.value)),
                }
            },
            false => {
                match &var.alias_false {
                    Some(alias) => Translate::WeAlreadyKnow.print(lang, alias.red(), None),
                    _ => Translate::WeAlreadyKnow.print(lang, value.to_string().purple().bold(), Some(var.value)),
                }
            },
        }
    }
}

pub fn print_history(history: String, variables: &HashMap<char, Variable>, query: char, lang: &Lang) {
    let paths: Vec<&str> = history.split('%').collect();
    if let Some(var) = variables.get(&query) {
        match var.value {
            true => {
                match &var.alias_true {
                    Some(alias) => Translate::WeKnow.print(&lang, alias.green(), None),
                    _ => Translate::WeKnow.print(&lang, query.to_string().purple().bold(), Some(var.value)),
                }
            },
            false => {
                match &var.alias_false {
                    Some(alias) => Translate::WeKnow.print(&lang, alias.red(), None),
                    _ => Translate::WeKnow.print(&lang, query.to_string().purple().bold(), Some(var.value)),
                }
            },
        }
    }
    for path in paths.iter() {
        if path.len() > 0 {
            match path.chars().next().unwrap() {
                'r' => print_rules_path(path.to_string(), variables, query, lang),
                'n' => print_false_no_rule(path.to_string(), variables, lang),
                'i' => print_already_know(path.to_string(), variables, lang),
                _ => unreachable!()
            }
        }
    }
}