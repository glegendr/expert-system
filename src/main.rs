mod algo;
mod models;
mod parsing;
mod leakser;
mod utils;

use std::collections::HashMap;
use parsing::{fill_maps, parse_line};
use models::Variable;
use algo::algo_v1;
use leakser::{leaks, Flag};
use utils::{print_variables, print_rules, tick_or_cross, print_variable};
use std::io;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use colored::{Colorize, ColoredString};

fn main() -> Result<(), String> {
    let (files, flags) = leaks()?;
    if flags.iter().any(|f| f == &Flag::Interactive) {
        interactive_mode(&files, &flags);
    } else {
        for file in files.iter() {
            let mut variables: HashMap<char, Variable> = HashMap::new();
            fill_maps(&mut variables, file)?;
            print_variables(&variables);
            algo_v1();
        }
    }
    Ok(())
}

fn interactive_mode(files: &Vec<String>, flags: &Vec<Flag>) {
    let mut variables: HashMap<char, Variable> = HashMap::new();
    for file in files {
        if let Err(e) = fill_maps(&mut variables, file) {
            println!("failed to parse {file}:\n{e}");
        }
    }
    let mut rl = Editor::<()>::new();
    let mut status = "ø".yellow();
    loop {
        let readline = rl.readline(&format!("{status}[expert-system] "));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut lower_line = line.to_lowercase();
                lower_line = lower_line.trim().to_string();
                match lower_line.as_str() {
                    "quit" => break,
                    "variables" | "var" => {
                        if variables.len() > 0 {
                            print_variables(&variables);
                        } else {
                            println!("no variables set");
                        }
                        status = tick_or_cross(true);
                    },
                    "rules" | "rule" => {
                        if variables.len() > 0 {
                            print_rules(&variables);
                        } else {
                            println!("no rules set");
                        }
                        status = tick_or_cross(true);
                    },
                    "exec" | "run" | "execute" => {
                        algo_v1();
                        status = tick_or_cross(true);
                    },
                    s => {
                        if let Some(variable) = variables.iter().find(|(k, v)| k.to_string() == line.clone() || v.alias_false == Some(line.clone()) || v.alias_true == Some(line.clone())) {
                            print_variable(variable);  
                            status = tick_or_cross(true);  
                        } else {
                            let chunks: Vec<&str> = line.split(" ").filter(|c| c.len() > 0).collect();
                            if let Some(key_word) = chunks.get(0) {
                                match key_word.to_lowercase().trim() {
                                    "file" => {
                                        if let Some(file) = chunks.get(1) {
                                            if let Err(e) = fill_maps(&mut variables, file) {
                                                println!("{e}");
                                                status = tick_or_cross(false);
                                            } else {
                                                println!("Ok");
                                                status = tick_or_cross(true);
                                            }
                                        } else {
                                            println!("no file provided");
                                            status = tick_or_cross(false);
                                        }
                                        continue
                                    }
                                    "remove" => {
                                        if let Some(kind) = chunks.get(1) {
                                            match kind.to_lowercase().trim() {
                                                "rule" | "rules" => status = tick_or_cross(remove_rule(chunks.get(2), &mut variables)),
                                                "var" | "variable" => status = tick_or_cross(remove_variable(chunks.get(2), &mut variables)),
                                                _ => {
                                                    println!("Expected one of [rule, var, variable] found {kind}");
                                                    status = tick_or_cross(false);
                                                }
                                            }
                                        }
                                        continue
                                    },
                                    _ => ()
                                }
                            }
                            match parse_line(&mut variables, line, true) {
                                Ok(()) => {
                                    println!("Ok");
                                    status = tick_or_cross(true);
                                },
                                Err(e) => {
                                    println!("{e}");
                                    status = tick_or_cross(false);
                                }
                            }
                            // println!("unknown command {line}");
                            // status = tick_or_cross(false);
                        }
                    },
                }
            },
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

fn rules_len(variables: &mut HashMap<char, Variable>) -> usize {
    let mut i = 0;
    for (_, v) in variables.iter() {
        i += v.rules.len();
    }
    i
}

fn remove_rule(nb_s: Option<&&str>, variables: &mut HashMap<char, Variable>) -> bool {
    if rules_len(variables) == 0 {
        println!("not any rules to delete");
        return false
    }
    match nb_s {
        Some(nb_s) => {
            match nb_s.parse::<usize>() {
                Ok(mut nb) => {
                    if nb <= 0 || nb > rules_len(variables) {
                        println!("expected a number between 1 and {}", rules_len(variables));
                        return false
                    }
                    let mut i = 0;
                    for (k, var) in variables.iter_mut() {
                        let index = nb - 1 - i;
                        if (index == 0 || index < var.rules.len()) && var.rules.len() > 0 {
                            println!("- {}", format!("{}", var.rules.get(index).unwrap()).red());
                            var.rules.remove(index);
                            return true
                        }
                        i += var.rules.len();
                    }
                    false
                },
                Err(_) => {
                    println!("{nb_s} is not a number");
                    false
                }
            }
        },
        None => {
            println!("expected a number found nothing");
            false
        }
    }
}

fn remove_variable(var_name: Option<&&str>, variables: &mut HashMap<char, Variable>) -> bool {
    match var_name {
        Some(var_name) => {
            let key = match variables.iter().find(|(k, v)| k.to_string() == String::from(*var_name) || v.alias_false == Some(String::from(*var_name)) || v.alias_true == Some(String::from(*var_name))) {
                Some((k, _)) => k.clone(),
                None => {
                    println!("cannot find variable {var_name}");
                    return false
                }
            };
            variables.remove(&key);
            for (k, variable) in variables.iter_mut() {
                variable.rules = variable.rules.iter().filter(|rule| !rule.formula_string.contains(|c| c == key)).cloned().collect();
            };
            true
        },
        None => {
            println!("expected a variable name found nothing");
            false
        }
    }
}
