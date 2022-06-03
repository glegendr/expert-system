use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use colored::Colorize;

const RESERVED_WORDS: [&'static str; 20] = [
    "and",
    "&",
    "+",
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

// #[derive(Debug)]
pub struct Rule {
    // input: fn(&HashMap<char, (Variable, Vec<Rule>)>) -> bool,
    // output: BTree, // A <=> B  [A, {...A}, {input A, output B}] [B, {...B}, {input B, output A}]
    formula_string: String
}


impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let mut ret = String::new();
        // for ope in &self.input {
        //     ret = format!("{ret}{}", ope.to_code_string());
        // }
        // ret = format!("{ret} {} ", if self.bidirectional {Operator::IfAndOnlyIf.to_code_string()} else {Operator::Then.to_code_string()});
        // for ope in &self.output {
        //     ret = format!("{ret}{}", ope.to_code_string());
        // }
        write!(f, "{}", self.formula_string)
    }
}

// #[derive(Debug)]
pub struct Variable {
    pub value: bool,
    pub locked: bool,
    pub requested: bool,
    pub alias_true: Option<String>,
    pub alias_false: Option<String>,
    pub rules: Vec<Rule>
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
            rules: Vec::default()
        }
    }

    pub fn request() -> Self {
        Variable {
            value: false,
            locked: false,
            requested: true,
            alias_true: None,
            alias_false: None,
            rules: Vec::default()
        }
    }

    pub fn insert() -> Self {
        Variable {
            value: true,
            locked: true,
            requested: false,
            alias_true: None,
            alias_false: None,
            rules: Vec::default()
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
        if let Some(var) = variables.get_mut(&string_to_char(chunk)) {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    And,
    Or,
    Xor,
    Equal,
    Not,
    Then,
    IfAndOnlyIf,
    Parentesis(bool),
    Var(char)
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
            "and" | "&" | "+" => Some(Operator::And),
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

    pub fn to_code_string(&self) -> String {
        match self {
            Operator::And => String::from("&"),
            Operator::Or => String::from("|"),
            Operator::Xor => String::from("^"),
            Operator::Equal => String::from("="),
            Operator::Not => String::from("!"),
            Operator::Then => String::from("=>"),
            Operator::IfAndOnlyIf => String::from("<=>"),
            Operator::Parentesis(true) => String::from("("),
            Operator::Parentesis(false) => String::from(")"),
            Operator::Var(s) => s.to_string(),
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


pub fn string_to_char(string: &str) -> char {
    string.chars().next().unwrap_or('/')
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
    // let _ = rules.insert(Rule {
    //     input: Operator::to_reverse_polish_notation(splited.get(0).unwrap_or(&Vec::new()))?,
    //     output: Operator::to_reverse_polish_notation(splited.get(1).unwrap_or(&Vec::new()))?,
    //     bidirectional: aritmetic.iter().find(|ope| *ope == &Operator::IfAndOnlyIf || *ope == &Operator::Then) == Some(&Operator::IfAndOnlyIf)
    // });
    //TODO to push in rules
    Ok(())
}

fn requests(chunks: &Vec<String>, variables: &mut HashMap<char, Variable>) -> Result<(), String> {
    for chunk in chunks.iter() {
        if let Some(var) = variables.get_mut(&string_to_char(chunk)) {
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
                        _ => Err(format!("{chunk} is not a valid variable name"))?,
                    };
                }
            }
        }

    }
    Ok(())
}

pub fn fill_maps(variables: &mut HashMap<char, Variable>, file: &str) -> Result<(), String> {
    let lines = to_splited_string(file)?;
    for line in lines {
        let chunks = line_to_chunk(&line)?;
        if let Some(first) = chunks.iter().next() {
            match (first.as_str(), string_to_char(first)){
                ("def", _) => def_var(&chunks[1..].to_vec(), variables)?,
                ("if", _) => def_rules(&chunks[1..].to_vec(), variables)?,
                ("=", _) => user_set(&chunks[1..].to_vec(), variables)?,
                (_, '=') => user_set(&chunks, variables)?,
                ("?", _) => requests(&chunks[1..].to_vec(), variables)?,
                (_, '?') => requests(&chunks, variables)?,
                _ => def_rules(&chunks, variables)?
            }
        }
    }
    Ok(())
}