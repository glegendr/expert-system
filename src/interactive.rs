use rustyline::error::ReadlineError;
use rustyline::Editor;
use colored::Colorize;
use std::process::Command;
use std::collections::HashMap;
use crate::utils::{print_variables, print_rules, tick_or_cross, print_variable};
use crate::parsing::{fill_maps, parse_line};
use crate::models::{Variable, Rule};
use crate::algo::{algo_v1, search_query};
use crate::leakser::Flag;

pub fn interactive_mode(files: &Vec<String>, _flags: &Vec<Flag>) {
    let mut variables: HashMap<char, Variable> = HashMap::new();
    for file in files {
        if let Err(e) = fill_maps(&mut variables, file, true) {
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
                        algo_v1(&mut variables);
                        status = tick_or_cross(true);
                    },
                    "clear" => {
                        variables.clear();
                        status = tick_or_cross(true);
                    },
                    _ => {
                        if let Some(variable) = variables.iter().find(|(k, v)| k.to_string() == line.clone() || v.alias_false == Some(line.clone()) || v.alias_true == Some(line.clone())) {
                            print_variable(variable);  
                            status = tick_or_cross(true);  
                        } else {
                            let chunks: Vec<&str> = line.split(" ").filter(|c| c.len() > 0).collect();
                            if let Some(key_word) = chunks.get(0) {
                                match key_word.to_lowercase().trim() {
                                    "help" => {
                                        let mut iter_chunk = chunks.iter();
                                        iter_chunk.next();
                                        status = tick_or_cross(helper(iter_chunk.collect()));
                                    },
                                    "pwd" | "ls" => {
                                        let mut command = Command::new(key_word.to_lowercase().trim());
                                        let mut iter_chunk = chunks.iter();
                                        iter_chunk.next();
                                        for chunk in iter_chunk {
                                            command.arg(chunk);
                                        }
                                        command.status().expect("failed to execute process");
                                        status = tick_or_cross(true);
                                    },
                                    "exec" | "run" | "execute" => {
                                        let mut iter_chunk = chunks.iter();
                                        let mut ret = true;
                                        iter_chunk.next();
                                        for var in iter_chunk {
                                            let var_name = match variables.iter().find(|(k, v)| k.to_string() == var.clone() || v.alias_false == Some(String::from(*var)) || v.alias_true == Some(String::from(*var))) {
                                                Some((k, _)) => *k,
                                                None => {
                                                    println!("{}", format!("{var} does not exist").red());
                                                    ret = false;
                                                    continue
                                                }
                                            };
                                            match search_query(var_name, &mut variables, &mut Vec::new()) {
                                                Ok(res) => println!("{var_name} is {res}"),
                                                Err(e) => println!("{e}")
                                            }
                                        }
                                        status = tick_or_cross(ret);
                                    },
                                    "file" => {
                                        if let Some(file) = chunks.get(1) {
                                            if let Err(e) = fill_maps(&mut variables, file, true) {
                                                println!("{e}");
                                                status = tick_or_cross(false);
                                            } else {
                                                status = tick_or_cross(true);
                                            }
                                        } else {
                                            println!("no file provided");
                                            status = tick_or_cross(false);
                                        }
                                    }
                                    "remove" | "del" | "delete" => {
                                        if let Some(kind) = chunks.get(1) {
                                            match kind.to_lowercase().trim() {
                                                "all" => {
                                                    variables.clear();
                                                    status = tick_or_cross(true);
                                                }
                                                "rule" | "rules" => status = tick_or_cross(remove_rule(chunks.get(2), &mut variables)),
                                                "var" | "variable" => status = tick_or_cross(remove_variable(chunks.get(2), &mut variables)),
                                                _ => {
                                                    println!("Expected one of [rule, var, variable, all] found {kind}");
                                                    status = tick_or_cross(false);
                                                }
                                            }
                                        }
                                    },
                                    _ => {
                                        match parse_line(&mut variables, line, true, false) {
                                            Ok(()) => {
                                                status = tick_or_cross(true);
                                            },
                                            Err(e) => {
                                                println!("{}", format!("{e}").red());
                                                status = tick_or_cross(false);
                                            }
                                        }
                                    }
                                }
                            }
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
                Ok(nb) => {
                    if nb <= 0 || nb > rules_len(variables) {
                        println!("expected a number between 1 and {}", rules_len(variables));
                        return false
                    }
                    let mut i = 0;
                    for (_, var) in variables.iter_mut() {
                        let index = nb - 1 - i;
                        if (index == 0 || index < var.rules.len()) && var.rules.len() > 0 {
                            println!("{}", format!("- {}", var.rules.get(index).unwrap()).red());
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
            println!("{}", format!("- {key}").red());
            variables.remove(&key);
            for (_, variable) in variables.iter_mut() {
                let (retain, filtered): (Vec<Rule>, Vec<Rule>) = variable.rules.clone().into_iter().partition(|rule| !rule.formula_string.contains(|c| c == key));
                variable.rules = retain;
                for filt in filtered {
                    println!("{}", format!("-{filt}").red());
                }

            };
            true
        },
        None => {
            println!("expected a variable name found nothing");
            false
        }
    }
}


fn helper(commands: Vec<&&str>) -> bool {
    let mut ret = true;
    if commands.len() == 0 {
        print_help();
        println!("");
        print_quit();
        println!("");
        print_variables_h();
        println!("");
        print_rules_h();
        println!("");
        print_clear();
        println!("");
        print_file();
        println!("");
        print_run();
        println!("");
        print_remove();
        println!("");
        print_equal();
        println!("");
        print_req();
        println!("");
        print_def();
        println!("");
        print_if();
    }
    for (i, command) in commands.into_iter().enumerate() {
        if i > 0 {
            println!("");
        }
        match *command {
            "help" => print_help(),
            "quit" => print_quit(),
            "variables" | "var" => print_variables_h(),
            "rules" | "rule" => print_rules_h(),
            "clear" => print_clear(),
            "file" => print_file(),
            "exec" | "run" | "execute" => print_run(),
            "remove" | "del" | "delete" => print_remove(),
            "=" => print_equal(),
            "?" => print_req(),
            "def" => print_def(),
            "if" => print_if(),
            "ls" | "pwd" => {
                Command::new("man").arg(command).status().expect("failed to execute process");
            }
            _ => {
                println!("unknown command {command}");
                ret = false;
            }
        }
    }
    ret
}

fn print_help() {println!("help <?Command ...>\n - display all commands or asked one")}
fn print_quit() { println!("quit\n - quit the program")}
fn print_variables_h() { println!("variables / var\n - list all variables and their rules")}
fn print_rules_h() { println!("rules / rule\n - list all rules")}
fn print_run() { println!("run <?Variable ...>\n - run the algorithm with variable if providen")}
fn print_clear() { println!("clear\n - alias for \"remove all\"")}
fn print_file() { println!("file <Path>\n - read the file in path and enrich variables and rules")}
fn print_remove() {
    println!("remove <kind> <?value>");
    println!("remove all\n - clear all variables and rules");
    println!("remove var <Variable>\n - remove the variable and all rules implicated");
    println!("remove rule <Index>\n - remove the rule depending the index listed with \"rules\"");
}
fn print_equal() { println!("= <Variable ...>\n - set the variable(s) to true")}
fn print_req() { println!("? <Variable ...>\n - set the variable(s) to requested")}
fn print_def() { println!("def <Variable> <?alias true> <?alias false>\n - create a new variable with name \"Variable\"")}
fn print_if() { println!("if <Rule>\n - create a new rule")}