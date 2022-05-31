use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use colored::Colorize;

const RESERVED_WORDS: [&'static str; 19] = [
    "and",
    "&",
    "or",
    "|",
    "xor",
    "^",
    "equal",
    "=",
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

#[derive(Debug)]
struct Variable {
    value: bool,
    locked: bool,
    requested: bool,
    alias_true: Option<String>,
    alias_false: Option<String>
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let is_ok = |v| if v {
            "✓".green()
        } else {
            "x".red()
        };
        write!(f, "{}|{}|{}|{}|{}", is_ok(self.value), is_ok(self.locked), is_ok(self.requested), self.alias_true.clone().unwrap_or_default(), self.alias_false.clone().unwrap_or_default())
    }
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
            1 => {
                if RESERVED_WORDS.contains(&chunk.as_str()) {
                    Err(format!("{chunk} is a reserved word"))?
                } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk || v.alias_false.clone().unwrap_or_default() == *chunk) {
                    Err(format!("{chunk} is already an alias for {k}"))?
                }
                var.alias_true = Some(String::from(chunk))
            },
            2 => {
                if RESERVED_WORDS.contains(&chunk.as_str()) {
                    Err(format!("{chunk} is a reserved word"))?
                } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk || v.alias_false.clone().unwrap_or_default() == *chunk) {
                    Err(format!("{chunk} is already an alias for {k}"))?
                }
                var.alias_false = Some(String::from(chunk))
            },
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

fn user_set(chunks: &Vec<String>, variables: &mut HashMap<String, Variable>) -> Result<(), String> {
    if chunks.len() == 1 {
        Err(String::from("= line expect at least 1 variable"))?
    }
    let mut iter_chunks = chunks.iter();
    iter_chunks.next();
    for chunk in iter_chunks {
        if let Some(var) = variables.get_mut(chunk) {
            var.value = true;
            var.locked = true;
        } else {
            for c in chunk.chars() {
                if let Some(var) = variables.get_mut(&c.to_string()) {
                    // If var.locked then warning
                    var.value = true;
                    var.locked = true;
                } else if let Some((_, var)) = variables.iter_mut().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {
                    var.value = true;
                    var.locked = true;
                } else {
                    variables.insert(c.to_string(), Variable::insert());
                }
            }
        }

    }
    // println!("{chunks:?}");
    Ok(())
}

#[derive(Debug, Clone)]
enum Operator {
    And,
    Or,
    Xor,
    Equal,
    Not,
    Then,
    IfAndOnlyIf,
    Parentesis(bool),
    Var(String)
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::IfAndOnlyIf => write!(f, "If-and-only-if"),
            Operator::Parentesis(true) => write!(f, "("),
            Operator::Parentesis(false) => write!(f, ")"),
            Operator::Var(s) => write!(f, "{s}"),
            _ => write!(f, "{}", format!("{self:?}")),
        }
    }
}

impl Operator {
    pub fn from_str(string: &str) -> Option<Operator> {
        match string {
            "and" | "&" => Some(Operator::And),
            "or" | "|" => Some(Operator::Or),
            "xor" | "^" => Some(Operator::Xor),
            "equal" | "=" => Some(Operator::Equal),
            "not" | "!" => Some(Operator::Not),
            "then" | "=>" => Some(Operator::Then),
            "if-and-only-if" | "<=>" => Some(Operator::IfAndOnlyIf),
            "(" => Some(Operator::Parentesis(true)),
            ")" => Some(Operator::Parentesis(false)),
            _ => None
        }
    }

    pub fn to_reverse_polish_notation(input: &Vec<Operator>) -> Result<Vec<Operator>, String> {
        let mut output: Vec<Operator> = Vec::new();
        let mut stack: Vec<Operator> = Vec::new();
        for operator in input.iter() {
            match operator {
                Operator::Var(_) => output.push(operator.clone()),
                Operator::Parentesis(false) => {
                    loop {
                        match stack.pop() {
                            Some(Operator::Parentesis(true)) => break,
                            Some(ope) => output.push(ope),
                            None => Err("unexpected closing delimiter: ')'")?
                        }
                    }
                },
                _ => {
                    if output.len() == 0 {
                        Err(format!("unexpected operator {operator}"))?
                    }
                    match operator {
                        Operator::IfAndOnlyIf | Operator::Then => {
                            loop {
                                match stack.pop() {
                                    Some(Operator::Parentesis(true)) => Err("unclosed delimiter '('")?,
                                    Some(ope) => output.push(ope),
                                    None => break
                                }
                            }
                            stack.push(operator.clone());
                        },
                        _ => stack.push(operator.clone())
                    }
                }
            }
        }
        loop {
            match stack.pop() {
                Some(Operator::Parentesis(true)) => Err("unclosed delimiter '('")?,
                Some(ope) => output.push(ope),
                None => break
            }
        }
        Ok(output)
    }
}


fn def_rules(chunks: &Vec<String>, variables: &mut HashMap<String, Variable>, rules: &mut HashSet<String>) -> Result<(), String> {
    let mut aritmetic: Vec<Operator> = Vec::new();
    let mut chunk_iter = chunks.iter();
    chunk_iter.next();
    for chunk in chunk_iter {
        if let Some(operator) = Operator::from_str(chunk) {
            aritmetic.push(operator);
        } else if let Some(_) = variables.get(chunk) {
            aritmetic.push(Operator::Var(String::from(chunk)));
        } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_true.clone().unwrap_or_default() == *chunk) {
            aritmetic.push(Operator::Var(String::from(k)));
        } else if let Some((k, _)) = variables.iter().find(|(_, v)| v.alias_false.clone().unwrap_or_default() == *chunk) {
            aritmetic.push(Operator::Not);
            aritmetic.push(Operator::Var(String::from(k)));
        } else if chunk.len() == 1 {
            match chunk.chars().next().unwrap() {
                'a'..='z' | 'A'..='Z' => {
                    variables.insert(chunk.clone(), Variable::default());
                    aritmetic.push(Operator::Var(String::from(chunk)));
                },
                _ => Err(format!("{chunk} is not a valid variable name"))?
            }
        } else {
            Err(format!("{chunk} is unexpected in rule line"))?
        }
    }
    println!("{:?}", Operator::to_reverse_polish_notation(&aritmetic)?);
    //TODO to push in rules
    Ok(())
}

fn fill_maps(variables: &mut HashMap<String, Variable>, rules: &mut HashSet<String>, file: &str) -> Result<(), String> {
    let lines = to_splited_string(file)?;
    for line in lines {
        println!("{:?}", line_to_chunk(&line));
        let chunks = line_to_chunk(&line)?;
        if let Some(first) = chunks.iter().next() {
            match first.as_str() {
                "def" => def_var(&chunks, variables)?,
                "if" => def_rules(&chunks, variables, rules)?,
                "=" => user_set(&chunks, variables)?,
                "?" => {},
                _ => {}
            }
        }
    }
    Ok(())

}

fn main() -> Result<(), String> {
    let file = "test_files/frog";
    let mut variables = HashMap::new();
    let mut rules = HashSet::new();
    fill_maps(&mut variables, &mut rules,  file)?;
    print_vars(&variables);
    // println!("Variables: {variables:?}\nRules:     {rules:?}");
    Ok(())
}

fn print_vars(variables: &HashMap<String, Variable>)  {
    println!("name v|l|r|true|false");
    for (k, v) in variables.iter() {
        println!("{k} -> {v}")
    }
}