use std::collections::HashMap;
use crate::models::{Variable, Operator};
use colored::Colorize;

fn print_false_no_rule(mut s: String, query: char) {
    let value: char = s.pop().unwrap();
    if value == query {
        println!("We know {} is {} because no rule assign it.", value.to_string().purple().bold(), "false".red());
    } else {
        println!("We know {} is {} because no rule assign it.", value.to_string().yellow().bold(), "false".red());
    }
}

fn print_rules_path(s: String, variables: &mut HashMap<char, Variable>, query: char) {
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
                if var.value == true {
                    if c == query {
                        print!("{} we know {} is {} ", conjuction_word, c.to_string().purple().bold(), var.value.to_string().green());
                    } else {
                        print!("{} we know {} is {} ", conjuction_word, c.to_string().yellow().bold(), var.value.to_string().green());
                    }
                } else {
                    if c == query {
                        print!("{} we know {} is {} ", conjuction_word, c.to_string().purple().bold(), var.value.to_string().red());
                    } else {
                        print!("{} we know {} is {} ", conjuction_word, c.to_string().yellow().bold(), var.value.to_string().red());
                    }
                }
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

pub fn print_history(history: String, variables: &mut HashMap<char, Variable>, query: char) {
    let paths: Vec<&str> = history.split('%').collect();
    if let Some(var) = variables.get(&query) {
        if var.value == true {
            println!("we know {} is {} because", query.to_string().purple().bold(), var.value.to_string().green());
        } else {
            println!("we know {} is {} because", query.to_string().purple().bold(), var.value.to_string().red());
        }
    }
    //println!("BITE {:?}", paths);
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

pub fn algo_v1(variables: &mut HashMap<char, Variable>) {
    let requested: Vec<char> = variables.iter().filter(|(_,v)| v.requested == true).map(|(k,_)| *k).collect();
    for c in requested {
        let history = String::new();
        match search_query(c, variables, &mut Vec::new(), history) {
            Ok((res, history)) => {
                if trace == true {
                    print_history(history, variables, c);
                } else {
                    println!("{} is {}", c, res);
                }
            },
            Err(e) => println!("{} => {}", c, e)
        }
    }
}

pub fn search_query(query: char, variables: &mut HashMap<char, Variable>, old_rules: &mut Vec<String>, mut history: String) -> Result<(bool, String), String> {
    for rule in variables.get(&query).unwrap().rules.iter() {
        if old_rules.contains(&rule.formula_string.clone()) == true {
            return Err("Error: the rule loop".to_string());
        }
    }

    if let Some(var) = variables.get(&query) {
        if var.locked == true {
            history.push_str("%i ");
            history.push(query);
            return Ok((var.value, history))
        }
    }
    let query_rules = variables.get(&query).unwrap().rules.clone();
        if query_rules.len() == 0 {
            if let Some(x) = variables.get_mut(&query) {
                x.value = false;
                x.locked = true;
            }
        history.push_str("%n ");
        history.push(query);
        return Ok((false, history))
    }

    let mut i = 0;
    while i < query_rules.len() {
        for c in query_rules[i].input.to_string().chars() {
            if c.is_alphabetic() == true {
                if variables.get(&c).unwrap().locked == false {
                    let mut new_vec = old_rules.clone();
                    if new_vec.contains(&query_rules[i].formula_string.clone()) == true {
                        return Err("Error: the rule loop".to_string());
                    } else {
                        new_vec.push(query_rules[i].formula_string.clone());
                    }
                    match search_query(c, variables, &mut new_vec, history.clone()) {
                        Ok((ret, h)) => {
                            history = h;
                            if let Some(x) = variables.get_mut(&c) {
                                x.value = ret;
                                if x.locked == false {
                                    x.locked = ret;
                                }
                            }
                        }
                        Err(e) => {
                            if i >= query_rules.len() - 1 {
                                return Err(e);
                            }
                        }
                    };
                }
            }
        }
        //HERE work output
        if query_rules[i].output.find_nodes(|ope| match ope {
            Operator::Xor => true,
            Operator::Equal => true,
            Operator::Material => true,
            Operator::Not => true,
            _ => false
        }).len() > 0 {
            //TODO find result with ^!>= operator in output
        } else if query_rules[i].output.find_nodes(|ope| match ope {
            Operator::Or => true,
            _ => false
        }).len() > 0 {
            /*
            TODO find result with | operator in output
            #A ^ B => C | D
            true -> if C = _;unlock then search_query(C) 
                if C = false;lock 
                then D = true;lock
                else if C = true;lock
                then search_query(D)
                    if D = _;unlock
                    then if D is Query then D = Err(undetermined value) then D = false;unlock
                    else if D = _;lock
                    then D = value;lock
                else  (C = _;unlock)
                then search_query(D)
                    if D = false;lock
                    then C = true;lock
                    else if D = true;lock
                    then C = if C is query then C = Err(undetermined value) then C = false;unlock
                    else  (D = _;unlock)
                    then C&|D if C&|D is query then C&|D Err(undetermined value) then C&|D = false;unlock
            false -> NIQ
            #A ^ B => A | C
            #A ^ B => C | D | E
            */
        } else {
            if query_rules[i].input.enrich(variables).eval() == true {
                for output_letter in query_rules[i].output.find_nodes(|l| match l {Operator::Var(_) => true, _ => false}) {
                    let letter = match output_letter {Operator::Var(v) => v, _ => unreachable!()};
                    if let Some(x) = variables.get_mut(&letter) {
                        x.value = true;
                        x.locked = true;
                    }
                }
                    history.push_str("%r");
                    history.push_str(&query_rules[i].formula_string.clone());
                    return Ok((true, history))
            }
        }
        i += 1;
    }
    if let Some(x) = variables.get_mut(&query) {
        x.value = false;
        //x.locked = false;
    }
    history.push_str("%r");
    history.push_str(&query_rules[i - 1].formula_string.clone());
    Ok((false, history))
}
