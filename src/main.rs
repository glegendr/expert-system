use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Variable {
    value: bool,
    locked: bool,
    requested: bool,
    alias_true: Option<String>,
    alias_false: Option<String>
}

impl Variable {
    pub fn default() -> Self {
        Variable {
            value: false,
            locked: false,
            requested: false,
            alias_true: None,
            alias_false: None,
        }
    }

    pub fn request() -> Self {
        Variable {
            value: false,
            locked: false,
            requested: true,
            alias_true: None,
            alias_false: None,
        }
    }

    pub fn insert() -> Self {
        Variable {
            value: true,
            locked: true,
            requested: false,
            alias_true: None,
            alias_false: None,
        }
    }
}

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
            ('"', false) => {
                scoped = true;
                acc.push(String::default())
            },
            ('"', true) => {
                scoped = false;
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

fn def_var(chunks: &Vec<String>, variables: &mut HashMap<String, Variable>) -> Result<(), String> {
    if chunks.len() == 1 {
        Err(String::from("def line expect variable"))?
    }
    let mut iter_chunks = chunks.iter();
    let mut var = Variable::default();
    let mut var_name = String::default();
    iter_chunks.next();
    for (i, chunk) in iter_chunks.enumerate() {
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
            1 => var.alias_true = Some(String::from(chunk)),
            2 => var.alias_false = Some(String::from(chunk)),
            _ => Err(format!("unexpected {chunk} in def line"))?
        }
    }
    if let Some(variable) = variables.get_mut(&var_name) {
        variable.alias_true = var.alias_true;
        variable.alias_false = var.alias_false;
    } else {
        drop(variables.insert(var_name, var));
    }
    Ok(())
}

fn fill_maps(variables: &mut HashMap<String, Variable>, rules: &mut HashSet<String>, file: &str) -> Result<(), String> {
    let lines = to_splited_string(file)?;
    for line in lines {
        println!("{:?}", line_to_chunk(&line));
        let chunks = line_to_chunk(&line)?;
        if let Some(first) = chunks.iter().next() {
            match first.as_str() {
                "def" => {def_var(&chunks, variables)?},
                "if" => {},
                "=" => {},
                "?" => {},
                _ => {}
            }
        }
        // match line.chars().next() {
        //     Some('?') => {
        //         let mut line_iter = line.chars();
        //         line_iter.next();
        //         for var in line_iter {
        //             match var {
        //                 'A'..='Z' | 'a'..='z' => {
        //                     if let Some(variable) = variables.get_mut(&var.to_string()) {
        //                         variable.requested = true;
        //                     } else {
        //                         drop(variables.insert(var.to_string(), Variable::request()));
        //                     }
        //                 },
        //                 _ => Err(format!("'{var}' is an invalid variable"))?
        //             }
        //         }
        //     },
        //     Some('=') => {
        //         let mut line_iter = line.chars();
        //         line_iter.next();
        //         for var in line_iter {
        //             match var {
        //                 'A'..='Z' | 'a'..='z' => {
        //                     if let Some(variable) = variables.get_mut(&var.to_string()) {
        //                         variable.value = true;
        //                         variable.locked = true;
        //                     } else {
        //                         drop(variables.insert(var.to_string(), Variable::insert()));
        //                     }
        //                 },
        //                 _ => Err(format!("'{var}' is an invalid variable"))?
        //             }
        //         }
        //     },
        //     _ => ()
        // }
    }
    Ok(())

}

fn main() -> Result<(), String> {
    let file = "frog";
    let mut variables = HashMap::new();
    let mut rules = HashSet::new();
    fill_maps(&mut variables, &mut rules,  file)?;
    println!("Variables: {variables:?}\nRules:     {rules:?}");
    Ok(())
}
