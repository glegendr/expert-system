use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use crate::{
    models::{Operator, Variable, Rule, BTree},
    utils::string_to_char,
};
use colored::Colorize;

const RESERVED_WORDS: [&'static str; 22] = [
    "and",
    "&",
    "+",
    "or",
    "|",
    "xor",
    "^",
    "equal",
    "=",
    "material",
    ">>",
    "not",
    "!",
    "then",
    "=>",
    "if-and-only-if",
    "<=>",
    "if",
    "def",
    "?",
    "(",
    ")"
];

fn rule_to_truth_table(rule: &Rule) -> Vec<HashMap<char, bool>> {
    let vars = rule.input.find_nodes(|n| match n {Operator::Var(_) => true, _ => false});
    let permutations: Vec<u32> = (0..=u32::MAX >> (32 - vars.len())).collect();
    let mut ret: Vec<HashMap<char, bool>> = Vec::new();
    for permutation in permutations {
        let mut variables = HashMap::new();
        for (i, var) in vars.iter().enumerate() {
            match var {
                Operator::Var(v) => drop(variables.insert(*v, (permutation >> i) & 1 == 1)),
                _ => ()
            }
        }
        if rule.input.enrich_bool(&variables).eval() {
            ret.push(variables);
        }
    }
    ret
}

fn check_loop_2(variables: &HashMap<char, Variable>, mut history: Vec<(char, Vec<HashMap<char, bool>>)>) -> Vec<(char, Vec<HashMap<char, bool>>)> {
    match history.pop() {
        Some((k, _)) => {
            if let Some(var) = variables.get(&k) {
                let mut v = Vec::new();
                for rule in &var.rules {
                    v.append(&mut rule_to_truth_table(rule));
                }
                history.push((k, v));
                for rule in &var.rules {
                    for c in rule.input.to_string().chars() {
                        if c.is_alphabetic() == true && !history.iter().any(|(k, _)| *k == c) {
                            let mut new_history = history.clone();
                            new_history.push((c, Vec::new()));
                            history = check_loop_2(&variables, new_history);
                        }
                    }
                }
            }
            return history
        },
        None => return history
    }
}

fn check_loop(variables: &HashMap<char, Variable>, rule: &Rule) -> Result<(), String>{
    let outuput_var = rule.output.find_nodes(|ope| match ope {Operator::Var(_) => true, _ => false});
    for var in outuput_var {
        match var {
            Operator::Var(k) => {
                let mut map: HashMap<char, HashMap<char, bool>> = HashMap::new();
                let ret_loop = check_loop_2(&variables, vec![(k, rule_to_truth_table(rule))]);
                for (output_letter, hashmap_vec) in &ret_loop {
                    for hashmap in hashmap_vec {
                        for input_letter in hashmap {
                            if let Some(x) = map.get_mut(&output_letter) {
                                if let Some(z) = x.insert(*input_letter.0, *input_letter.1) {
                                    if z != *input_letter.1 {
                                        if let Some((_, error_vec)) = ret_loop.iter().find(|(l_output_letter, _)| l_output_letter == input_letter.0) {
                                            if let Some(_) = error_vec.iter().find(|loop_map| loop_map.get(&output_letter).is_some()) {
                                                Err(format!("contradiction in rule{rule}"))?
                                            }
                                        }
                                    }
                                }
                            } else {
                                map.insert(*output_letter, hashmap.clone());
                            }
                        }
                    }
                }
            },
            _ => ()
        }
    }
    Ok(())
}

/* ---------- STRING TRANSFORMATIONS ---------- */
fn to_splited_string(file: &str) -> Result<Vec<String>, String> {
    let mut file = File::open(file).map_err(|e| format!("{e}"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("{e}"))?;
    Ok(contents.split("\n").fold(Vec::new(), |mut acc, line| {
        let without_comment = line.split("#").next().unwrap_or_default();
        if without_comment.len() > 0 {
            acc.push(String::from(without_comment.trim()));
        }
        acc
    }))
}

fn line_to_chunk(line: &str) -> Result<Vec<String>, String> {
    let (mut ret, scoped) = line.chars().fold((vec![String::default()], false), |(mut acc, mut scoped), c| {
        let accessor = acc.len() - 1;
        match (c, scoped) {
            (' ', false) => acc.push(String::default()),
            ('>', false) => {
                if let Some(last_chunk) = acc.get_mut(accessor) {
                    match last_chunk.as_str() {
                        "=" | ">" | "<=" => {
                            last_chunk.push('>');
                            acc.push(String::default())
                        }
                        _ => acc.push(String::from('>'))
                    }
                }
            }
            ('=', false) => {
                if let Some(last_chunk) = acc.get_mut(accessor) {
                    match last_chunk.as_str() {
                        "<" => last_chunk.push('='),
                        _ => acc.push(String::from('='))
                    }
                }
            },
            ('!' | '+' | '^'| '&' | '(' | ')' | '|', false) => {
                acc.push(String::from(c));
                acc.push(String::default())
            },
            ('"', false) => {
                scoped = true;
                acc.push(String::default())
            },
            ('"', true) => {
                scoped = false;
                acc.push(String::default())
            },
            _ => acc.get_mut(accessor).unwrap().push(c),
        };
        (acc, scoped)
    });
    if scoped {
        Err(format!("unclosed quote in line: {line}"))?
    }
    Ok(ret.iter_mut()
        .map(|chunk| String::from(chunk.trim()))
        .filter(|chunk| chunk.len() > 0)
        .collect())
}

/* ---------- FILLING VARIABLES ---------- */
fn def_var(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>, silence: bool) -> Result<(), String> {
    if chunks.len() == 0 {
        Err(String::from("def line expect variable"))?
    }
    let mut var = Variable::default();
    let mut var_name = String::default();
    for (i, chunk) in chunks.iter().enumerate() {
        match i {
            0 => {
                if chunk.len() != 1 {
                    Err(format!("{chunk} is not a valid variable name"))?
                } else if let Some(name) = chunk.chars().next() {
                    match name.is_alphabetic() {
                        true => var_name = chunk.clone(),
                        _ => Err(format!("{chunk} is not a valid variable name"))?
                    }
                }
            },
            1 => {
                if RESERVED_WORDS.contains(&chunk.as_str()) {
                    Err(format!("{chunk} is a reserved word"))?
                } else if let Some((k, _)) = variables.iter().find(|(k, v)| k.to_string() != var_name && (v.alias_true.clone().unwrap_or_default() == *chunk || v.alias_false.clone().unwrap_or_default() == *chunk)) {
                    Err(format!("{chunk} is already an alias for {k}"))?
                } else if chunk.len() < 2 {
                    Err(format!("{chunk} is too short to be an alias"))?
                }
                var.alias_true = Some(String::from(chunk))
            },
            2 => {
                if RESERVED_WORDS.contains(&chunk.as_str()) {
                    Err(format!("{chunk} is a reserved word"))?
                } else if let Some((k, _)) = variables.iter().find(|(k, v)| k.to_string() != var_name && (v.alias_true.clone().unwrap_or_default() == *chunk || v.alias_false.clone().unwrap_or_default() == *chunk)) {
                    Err(format!("{chunk} is already an alias for {k}"))?
                } else if var.alias_true == Some(String::from(chunk)) {
                    Err(format!("{chunk} is already the true alias"))?
                } else if chunk.len() < 2 {
                    Err(format!("{chunk} is too short to be an alias"))?
                }
                var.alias_false = Some(String::from(chunk))
            },
            _ => Err(format!("unexpected {chunk} in def line"))?
        }
    }
    if let Some((_, variable)) = variables.iter_mut().find(|(k, _)| k.to_string() == var_name) {
        if !silence {
            println!("{}", format!("- {var_name} => {variable}").red());
        }
        variable.alias_true = var.alias_true;
        variable.alias_false = var.alias_false;
        if !silence {
            println!("{}", format!("+ {var_name} => {variable}").green());
        }
    } else {
        if !silence {
            println!("{}", format!("+ {var_name} => {var}").green());
        }
        drop(variables.insert(string_to_char(&var_name), var));
    }
    Ok(())
}

fn user_set(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>, silence: bool) -> Result<(), String> {
    for chunk in chunks.iter() {
        if let Some((k, var)) = variables.iter_mut().find(|(_, v)| v.alias_true == Some(String::from(chunk))) {
            if !silence {
                println!("{}", format!("+= {k}").green());
            }
            var.value = true;
            var.locked = true;
        } else {
            for c in chunk.chars() {
                if let Some(var) = variables.get_mut(&c) {
                    if !silence {
                        println!("{}", format!("+= {c}").green());
                    }
                    var.value = true;
                    var.locked = true;
                } else if let Some((k, var)) = variables.iter_mut().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {   
                    if !silence {
                        println!("{}", format!("+= {k}").green());
                    }
                    var.value = true;
                    var.locked = true;
                } else {
                    match c.is_alphabetic() {
                        true => {
                            if !silence {
                                println!("{}", format!("+= {c}").green());
                            }
                            variables.insert(c, Variable::insert());
                        },
                        _ => Err(format!("{c} is an invalid variable name"))?
                    }
                }
            }
        }

    }
    Ok(())
}

fn aritmetic_to_string(aritmetic: &Vec<Vec<Operator>>, reverse: bool) -> String {
    aritmetic
        .iter()
        .fold(String::new(), |acc, v| {
            let v_s = v.iter().fold(String::new(), |acc, ope| format!("{acc} {ope}"));
            if acc.len() == 0 {
                return v_s
            }
            match reverse {
                true => format!("{v_s} =>{acc}", ),
                false => format!("{acc} =>{v_s}")
            }
        })

}

fn check_splited(splited: &Vec<Vec<Operator>>) -> Result<(), String> {
    for part in splited {
        let mut last_op = None;
        for operator in part {
            match (operator, &last_op) {
                (Operator::Var(_) | Operator::Parentesis(true), None | Some(Operator::Or) | Some(Operator::Material) | Some(Operator::And) | Some(Operator::Xor) | Some(Operator::Equal) | Some(Operator::Not) | Some(Operator::Parentesis(true))) |
                (Operator::Or | Operator::And | Operator::Xor | Operator::Equal | Operator::Material, Some(Operator::Var(_)) | Some(Operator::Parentesis(false))) |
                (Operator::Not, None | Some(Operator::Or) | Some(Operator::And) | Some(Operator::Xor) | Some(Operator::Equal)) |
                (Operator::Parentesis(false), Some(Operator::Parentesis(false)) | Some(Operator::Var(_))) => {
                    last_op = Some(operator.clone());
                },
                (_, None) => Err(format!("Unexpected operator {operator}"))?,
                (_, Some(last_op)) => Err(format!("Unexpected operator {operator} following {last_op}"))?
            }
        }
    }
    Ok(())
}

fn insert_rule(rule: &Rule, variables: &mut HashMap<char, Variable>, silence: bool) -> Result<(), String> {
    if rule.output.find_nodes(|n| match n {Operator::Var(_) | Operator::And => false, _ => true}).len() > 0 {
        Err("output can only handle & and variables")?
    }
    let outuput_var = rule.output.find_nodes(|ope| match ope {Operator::Var(_) => true, _ => false});
    let input_var = rule.input.find_nodes(|ope| match ope {Operator::Var(_) => true, _ => false});
    for var in input_var {
        if outuput_var.contains(&var) {
            Err(format!("{var} is both in input and output"))?
        }
    }
    if !silence {
        println!("{}", format!("+{rule}").green());
    }
    for var in outuput_var {
        if let Operator::Var(v) = var {
            if let Some(var) = variables.get_mut(&v) {
                if !var.rules.iter().any(|ru| ru.formula_string == rule.formula_string) {
                    var.rules.push(rule.clone());
                }
            }
        }
    }
    check_loop(&variables, rule)?;
    Ok(())
}

fn def_rules(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>, silence: bool) -> Result<(), String> {
    let mut aritmetic: Vec<Operator> = Vec::new();
    for chunk in chunks.iter() {
        if let Some(operator) = Operator::from_str(chunk) {
            aritmetic.push(operator);
        } else if let Some(_) = variables.iter().find(|(k, _)| k.to_string() == *chunk) {
            aritmetic.push(Operator::Var(string_to_char(chunk)));
        } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {
            aritmetic.push(Operator::Var(*k));
        } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_false.clone().unwrap_or_default() == *chunk) {
            aritmetic.push(Operator::Not);
            aritmetic.push(Operator::Var(*k));
        } else if chunk.len() == 1 {
            match chunk.chars().next().unwrap().is_alphabetic() {
                true => {
                    variables.insert(string_to_char(chunk), Variable::default());
                    if !silence {
                        println!("{}", format!("+ {}", string_to_char(chunk)).green());
                    }
                    aritmetic.push(Operator::Var(string_to_char(chunk)));
                },
                _ => Err(format!("{chunk} is not a valid variable name "))?
            }
        } else {
            Err(format!("{chunk} is unexpected in rule line"))?
        }
    }
    let splited: Vec<Vec<Operator>> = aritmetic.split(|ope| *ope == Operator::IfAndOnlyIf || *ope == Operator::Then).map(|ope| ope.to_vec()).collect();
    match splited.len() {
        0 => Err("unexpected error")?,
        1 => Err("expected => or <=> operator")?,
        2 => (),
        _ => Err("expected only 1 => or <=> operator")?
    }
    check_splited(&splited)?;
    let rule = Rule {
        input: BTree::from_vec(&mut Operator::to_reverse_polish_notation(splited.get(0).unwrap_or(&Vec::new()))?)?,
        output: BTree::from_vec(&mut Operator::to_reverse_polish_notation(splited.get(1).unwrap_or(&Vec::new()))?)?,
        formula_string: aritmetic_to_string(&splited, false)
    };
    insert_rule(&rule, variables, silence)?;
    if let Some(Operator::IfAndOnlyIf) = aritmetic.iter().find(|ope| *ope == &Operator::IfAndOnlyIf) {
        let rule_2 = Rule {
            input: rule.output.clone(),
            output: rule.input.clone(),
            formula_string: aritmetic_to_string(&splited, true)
        };
        insert_rule(&rule_2, variables, silence)?;
    }
    Ok(())
}

fn requests(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>, silence: bool) -> Result<(), String> {
    for chunk in chunks.iter() {
        if let Some((_, var)) = variables.iter_mut().find(|(_, v)| v.alias_true == Some(String::from(chunk))) {
            var.requested = true;
        } else {
            for c in chunk.chars() {
                if let Some(var) = variables.get_mut(&c) {
                    if !silence {
                        println!("{}", format!("+? {c}").green());
                    }
                    var.requested = true;
                } else if let Some((k, var)) = variables.iter_mut().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {
                    if !silence {
                        println!("{}", format!("+? {k}").green());
                    }
                    var.requested = true;
                } else {
                    match c.is_alphabetic() {
                        true => {
                            variables.insert(c, Variable::request());
                            if !silence {
                                println!("{}", format!("+? {c}").green());
                            }
                        },
                        _ => Err(format!("{c} is not a valid variable name"))?,
                    };
                }
            }
        }

    }
    Ok(())
}

pub fn parse_line(old_variables: &mut HashMap<char, Variable>, line: String, restricted: bool, silence: bool) -> Result<(), String> {
    let mut variables = (*old_variables).clone();
    let mut chunks = line_to_chunk(&line)?;
    if let Some(first) = chunks.iter().next() {
        match (first.as_str(), string_to_char(first)){
            ("def", _) => def_var(&chunks[1..].to_vec(), &mut variables, silence)?,
            ("if", _) => def_rules(&chunks[1..].to_vec(), &mut variables, silence)?,
            ("=", _) => user_set(&chunks[1..].to_vec(), &mut variables, silence)?,
            (_, '=') => {
                if let Some(first) = chunks.get_mut(0) {
                    first.remove(0);
                }
                user_set(&chunks, &mut variables, silence)?
            },
            ("?", _) => requests(&chunks[1..].to_vec(), &mut variables, silence)?,
            (_, '?') => {
                if let Some(first) = chunks.get_mut(0) {
                    first.remove(0);
                }
                requests(&chunks, &mut variables, silence)?
            },
            _ => {
                if restricted {
                    Err(format!("Expected one of [=, ?, def, if] found {line}"))?
                } else {
                    def_rules(&chunks, &mut variables, silence)?
                }
            }
        }
    }
    *old_variables = variables;
    Ok(())
}

pub fn fill_maps(variables: &mut HashMap<char, Variable>, file: &str, silence: bool) -> Result<(), String> {
    let lines = to_splited_string(file)?;
    for line in lines {
       parse_line(variables, line, false, silence)?;
    }
    Ok(())
}