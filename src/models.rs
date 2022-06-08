use std::fmt;
use crate::utils::tick_or_cross;
use std::collections::HashMap;

/* ---------- RULE ---------- */
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub input: BTree,
    pub output: BTree,
    pub formula_string: String
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.formula_string)
    }
}

/* ---------- VARIABLE ---------- */
#[derive(Debug, Clone)]
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
        write!(
            f,
            "[{}] [{}] [{}] [{}] [{}]",
            tick_or_cross(self.value),
            tick_or_cross(self.locked),
            tick_or_cross(self.requested),
            self.alias_true.as_ref().unwrap_or(&String::default()),
            self.alias_false.as_ref().unwrap_or(&String::default())
        )
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

/* ---------- OPERATOR ---------- */ 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    And,
    Or,
    Xor,
    Equal,
    Material,
    Not,
    Then,
    IfAndOnlyIf,
    Parentesis(bool),
    Var(char),
    B(bool)
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::And =>  write!(f, "&"),
            Operator::Or =>  write!(f, "|"),
            Operator::Xor =>  write!(f, "^"),
            Operator::Equal =>  write!(f, "="),
            Operator::Material =>  write!(f, ">>"),
            Operator::Not =>  write!(f, "!"),
            Operator::Then =>  write!(f, "=>"),
            Operator::IfAndOnlyIf =>  write!(f, "<=>"),
            Operator::Parentesis(true) => write!(f, "("),
            Operator::Parentesis(false) => write!(f, ")"),
            Operator::Var(s) => write!(f, "{s}"),
            Operator::B(true) =>  write!(f, "1"),
            Operator::B(false) =>  write!(f, "0")
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
            "material" | ">>" => Some(Operator::Material),
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
                Operator::Not | Operator::Parentesis(true) => stack.push(operator.clone()),
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

/* ---------- BTREE ---------- */

#[derive(Debug, Clone, PartialEq)]
pub struct BTree {
    pub c1: Option<Box<BTree>>,
    pub c2: Option<Box<BTree>>,
    pub node: Operator
}

impl fmt::Display for BTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{}", self.to_string()))
    }
}

impl BTree {
    pub fn new(node: Operator) -> BTree {
        BTree {
            c1: None,
            c2: None,
            node,
        }
    }

    pub fn insert_a(&mut self, sub_tree: BTree) {
        self.c1 = Some(Box::new(sub_tree));
    }

    pub fn insert_b(&mut self, sub_tree: BTree) {
        self.c2 = Some(Box::new(sub_tree));
    }

    pub fn to_string(&self) -> String {
        match (&self.node, &self.c1, &self.c2) {
            (Operator::And, Some(c1), Some(c2)) => format!("{c1}{c2}&"),
            (Operator::Or, Some(c1), Some(c2)) => format!("{c1}{c2}|"),
            (Operator::Xor, Some(c1), Some(c2)) => format!("{c1}{c2}^"),
            (Operator::Equal, Some(c1), Some(c2)) => format!("{c1}{c2}="),
            (Operator::Material, Some(c1), Some(c2)) => format!("{c1}{c2}>>"),
            (Operator::Not, Some(c1), None) => format!("{c1}!"),
            (Operator::Not, None, Some(c2)) => format!("{c2}!"),
            (Operator::B(b), _, _) => format!("{b}"),
            (Operator::Var(v), _, _) => format!("{v}"),
            _ => format!("{self:?}")
        }
    }

    pub fn from_vec(formula: &mut Vec<Operator>) -> Result<BTree, String> {
        if let Some(last_op) = formula.pop() {
            let mut ret = match last_op {
                Operator::B(boole) => return Ok(BTree::new(Operator::B(boole))),
                Operator::Var(v) => return Ok(BTree::new(Operator::Var(v))),
                Operator::Parentesis(_) | Operator::IfAndOnlyIf | Operator::Then => Err("unexpected operator {last_op} in btree")?,
                op => BTree::new(op)
            };
            ret.insert_b(BTree::from_vec(formula)?);
            if ret.node != Operator::Not {
                ret.insert_a(BTree::from_vec(formula)?);
            }
            return Ok(ret)
        }
        Err(String::from("Error while parsing formula"))
    }

    pub fn eval(&self) -> bool {
        match calc_formula(&Box::new(self.clone())) {
            Ok(res) => res,
            Err(e) => {
                println!("{e}");
                false
            }
        }
    }

    pub fn find_nodes(&self, serch_fn: fn (&Operator) -> bool) -> Vec<Operator> {
        let mut ret: Vec<Operator> = Vec::new();

        if serch_fn(&self.node) {
            ret.push(self.node.clone());
        }
        if let Some(child_1) = &self.c1 {
            ret.append(&mut child_1.find_nodes(serch_fn));
        }
        if let Some(child_2) = &self.c2 {
            ret.append(&mut child_2.find_nodes(serch_fn));
        }

        ret
    }

    pub fn enrich(&self, variables: &HashMap<char, Variable>) -> BTree {
        match &self.node {
            Operator::Var(c) => {
                if let Some(var) = variables.get(c) {
                    BTree::new(Operator::B(var.value))    
                } else {
                    BTree::new(Operator::B(false))
                }
            },
            Operator::B(c) => BTree::new(Operator::B(*c)),
            _ => {
                let mut ret = BTree::new(self.node.clone());
                if let Some(c1) = &self.c1 {
                    ret.c1 = Some(Box::new(c1.enrich(variables)));
                }
                if let Some(c2) = &self.c2 {
                    ret.c2 = Some(Box::new(c2.enrich(variables)));
                }
                ret
            }
        }
    }
}

fn calc_formula(tree: &Box<BTree>) -> Result<bool, String> {
    match (&tree.node, &tree.c1, &tree.c2) {
        (Operator::And, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? & calc_formula(&c2)?),
        (Operator::Or, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? | calc_formula(&c2)?),
        (Operator::Xor, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? ^ calc_formula(&c2)?),
        (Operator::Material, Some(c1), Some(c2)) => Ok(!(calc_formula(&c1)? && !calc_formula(&c2)?)),
        (Operator::Equal, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? == calc_formula(&c2)?),
        (Operator::Not, Some(c1), None) => Ok(!calc_formula(&c1)?),
        (Operator::Not, None, Some(c2)) => Ok(!calc_formula(&c2)?),
        (Operator::B(b), None, None) => Ok(*b),
        _ => {
            return Err(String::from("Error while calculating formula"))
        }
    }
}
