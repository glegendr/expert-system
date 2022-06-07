use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use crate::{
    models::{Operator, Variable, Rule, BTree},
    utils::string_to_char,
};

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

/* ---------- STRUCTS ---------- */

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
            ('!' | '+' | '^'| '&', false) => {
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
fn def_var(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>  ) -> Result<(), String> {
    if chunks.len() == 1 {
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
                    match name {
                        'A'..='Z' | 'a'..='z' => var_name = chunk.clone(),
                        _ => Err(format!("{chunk} is not a valid variable name"))?
                    }
                }
            },
            1 => {
                if RESERVED_WORDS.contains(&chunk.as_str()) {
                    Err(format!("{chunk} is a reserved word"))?
                } else if let Some((k, _)) = variables.iter().find(|(k, v)| k.to_string() != var_name && (v.alias_true.clone().unwrap_or_default() == *chunk || v.alias_false.clone().unwrap_or_default() == *chunk)) {
                    Err(format!("{chunk} is already an alias for {k}"))?
                }
                var.alias_true = Some(String::from(chunk))
            },
            2 => {
                if RESERVED_WORDS.contains(&chunk.as_str()) {
                    Err(format!("{chunk} is a reserved word"))?
                } else if let Some((k, _)) = variables.iter().find(|(k, v)| k.to_string() != var_name && (v.alias_true.clone().unwrap_or_default() == *chunk || v.alias_false.clone().unwrap_or_default() == *chunk)) {
                    Err(format!("{chunk} is already an alias for {k}"))?
                }
                var.alias_false = Some(String::from(chunk))
            },
            _ => Err(format!("unexpected {chunk} in def line"))?
        }
    }
    if let Some(variable) = variables.get_mut(&string_to_char(&var_name)) {
        variable.alias_true = var.alias_true;
        variable.alias_false = var.alias_false;
    } else {
        drop(variables.insert(string_to_char(&var_name), var));
    }
    Ok(())
}

fn user_set(chunks: &Vec<String>, variables: &mut HashMap<char, Variable> ) -> Result<(), String> {
    for chunk in chunks.iter() {
        if let Some((_, var)) = variables.iter_mut().find(|(_, v)| v.alias_true == Some(String::from(chunk))) {
            var.value = true;
            var.locked = true;
        } else {
            for c in chunk.chars() {
                if let Some(var) = variables.get_mut(&c) {
                    var.value = true;
                    var.locked = true;
                } else if let Some((_, var)) = variables.iter_mut().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {
                    var.value = true;
                    var.locked = true;
                } else {
                    variables.insert(c, Variable::insert());
                }
            }
        }

    }
    Ok(())
}

fn def_rules(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>) -> Result<(), String> {
    let mut aritmetic: Vec<Operator> = Vec::new();
    for chunk in chunks.iter() {
        if let Some(operator) = Operator::from_str(chunk) {
            aritmetic.push(operator);
        } else if let Some(_) = variables.get(&string_to_char(chunk)) {
            aritmetic.push(Operator::Var(string_to_char(chunk)));
        } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {
            aritmetic.push(Operator::Var(*k));
        } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_false.clone().unwrap_or_default() == *chunk) {
            aritmetic.push(Operator::Not);
            aritmetic.push(Operator::Var(*k));
        } else if chunk.len() == 1 {
            match chunk.chars().next().unwrap() {
                'a'..='z' | 'A'..='Z' => {
                    variables.insert(string_to_char(chunk), Variable::default());
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
    let mut rule = Rule {
        input: BTree::from_vec(&mut Operator::to_reverse_polish_notation(splited.get(0).unwrap_or(&Vec::new()))?)?,
        output: BTree::from_vec(&mut Operator::to_reverse_polish_notation(splited.get(1).unwrap_or(&Vec::new()))?)?,
        formula_string: String::from("")
    };
    rule.formula_string = format!("{} => {}", rule.input, rule.output);
    for var in rule.output.find_nodes(|ope| match ope {Operator::Var(_) => true, _ => false}) {
        if let Operator::Var(v) = var {
            if let Some(var) = variables.get_mut(&v) {
                var.rules.push(rule.clone());
            }
        }
    }
    if let Some(Operator::IfAndOnlyIf) = aritmetic.iter().find(|ope| *ope == &Operator::IfAndOnlyIf) {
        let mut rule_2 = Rule {
            input: rule.output.clone(),
            output: rule.input.clone(),
            formula_string: String::from("")
        };
        rule_2.formula_string = format!("{} => {}", rule_2.input, rule_2.output);
        for var in rule_2.output.find_nodes(|ope| match ope {Operator::Var(_) => true, _ => false}) {
            if let Operator::Var(v) = var {
                if let Some(var) = variables.get_mut(&v) {
                    var.rules.push(rule_2.clone());
                }
            }
        }
    }
    Ok(())
}

fn requests(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>) -> Result<(), String> {
    for chunk in chunks.iter() {
        if let Some((_, var)) = variables.iter_mut().find(|(_, v)| v.alias_true == Some(String::from(chunk))) {
            var.requested = true;
        } else {
            for c in chunk.chars() {
                if let Some(var) = variables.get_mut(&c) {
                    var.requested = true;
                } else if let Some((_, var)) = variables.iter_mut().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {
                    var.requested = true;
                } else {
                    match c {
                        'A'..='Z' | 'a'..='z' => variables.insert(c, Variable::request()),
                        _ => Err(format!("{c} is not a valid variable name"))?,
                    };
                }
            }
        }

    }
    Ok(())
}

pub fn parse_line(variables: &mut HashMap<char, Variable>, line: String, restricted: bool) -> Result<(), String> {
    let mut chunks = line_to_chunk(&line)?;
    if let Some(first) = chunks.iter().next() {
        match (first.as_str(), string_to_char(first)){
            ("def", _) => def_var(&chunks[1..].to_vec(), variables)?,
            ("if", _) => def_rules(&chunks[1..].to_vec(), variables)?,
            ("=", _) => user_set(&chunks[1..].to_vec(), variables)?,
            (_, '=') => {
                if let Some(first) = chunks.get_mut(0) {
                    first.remove(0);
                }
                user_set(&chunks, variables)?
            },
            ("?", _) => requests(&chunks[1..].to_vec(), variables)?,
            (_, '?') => {
                if let Some(first) = chunks.get_mut(0) {
                    first.remove(0);
                }
                requests(&chunks, variables)?
            },
            _ => {
                if restricted {
                    Err(format!("Expected one of [=, ?, def, if] found {line}"))?
                } else {
                    def_rules(&chunks, variables)?
                }
            }
        }
    }
    Ok(())
}

pub fn fill_maps(variables: &mut HashMap<char, Variable>, file: &str) -> Result<(), String> {
    let lines = to_splited_string(file)?;
    for line in lines {
       parse_line(variables, line, false)?;
    }
    Ok(())
}