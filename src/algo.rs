use std::collections::HashMap;
use crate::models::{Variable, Operator};

pub fn algo_v1(variables: &mut HashMap<char, Variable>) {
    let requested: Vec<char> = variables.iter().filter(|(_,v)| v.requested == true).map(|(k,_)| *k).collect();
    for c in requested {
        match search_query(c, variables, &mut Vec::new()) {
            Ok(res) => println!("{} is {}", c, res),
            Err(e) => println!("{} => {}", c, e)
        }
    }
}

pub fn search_query(query: char, variables: &mut HashMap<char, Variable>, old_rules: &mut Vec<String>) -> Result<bool, String> {
    for rule in variables.get(&query).unwrap().rules.iter() {
        if old_rules.contains(&rule.formula_string.clone()) == true {
            return Err("Error: the rule loop".to_string());
        }
    }
    for rule in variables.get(&query).unwrap().rules.iter() {
        old_rules.push(rule.formula_string.clone());
    }

    if let Some(var) = variables.get(&query) {
        if var.locked == true {
            return Ok(var.value)
        }
    }
    let query_rules = variables.get(&query).unwrap().rules.clone();
        if query_rules.len() == 0 {
            if let Some(x) = variables.get_mut(&query) {
                x.value = false;
                x.locked = true;
            }
        return Ok(false)
    }

    let mut i = 0;
    while i < query_rules.len() {
        for c in query_rules[i].input.to_string().chars() {
            if c.is_alphabetic() == true {
                if variables.get(&c).unwrap().locked == false {
                    match search_query(c, variables, old_rules) {
                        Ok(b) => {
                            if let Some(x) = variables.get_mut(&c) {
                                x.value = b;
                                x.locked = b;
                            }
                        }
                        Err(_) => (),
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
                    return Ok(true)
            }
        }
        i += 1;
    }
    if let Some(x) = variables.get_mut(&query) {
        x.value = false;
        //x.locked = false;
    }
    Ok(false)
}
