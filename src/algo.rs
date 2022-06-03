use std::collections::HashMap;
use crate::operator::eval_formula;

pub fn algo_v1() {
    let mut rules: Vec<String> = Vec::new();
    rules.push(String::from("A B! & => F"));
    rules.push(String::from("B A! & => C"));
    rules.push(String::from("A B & => M"));
    rules.push(String::from("F M | => G"));
    rules.push(String::from("C => Y"));
    let mut facts = HashMap::new();
    facts.insert('A',(true, true));
    facts.insert('B',(true, true));
    facts.insert('C',(false, false));
    facts.insert('F',(false, false));
    facts.insert('M',(false, false));
    facts.insert('G',(false, false));
    facts.insert('Y',(false, false));
    println!("{}", search_querie('Y', &mut facts, &rules));
    println!("{}", search_querie('G', &mut facts, &rules));
}

fn search_querie(querie: char, facts: &mut HashMap<char, (bool, bool)>, rules: &Vec<String>) -> bool {
    match facts.get_key_value(&querie) {
        Some((_, (value, lock))) => {
            if *lock == true {
                return *value
            }
        }
        _ => ()
    }
    //TO UP -> check all seconds parts of RULES & keep RULES with querie in second part
    let mut querie_rules = Vec::new();
    for rule in rules.iter() {
        if rule.chars().last() == Some(querie) {
            querie_rules.push(rule);
        }
    }
    if querie_rules.len() == 0 {
        facts.remove(&querie);
        facts.insert(querie, (false, true));
        return false
    }
    //TODO -> replace querie_rules[0] to function who choose the simpliest
    for c in querie_rules[0].chars() {
        if c.is_alphabetic() == true {
            if let Some((_, lock)) = facts.get_mut(&c) {
                if *lock == false {
                    facts.insert(c, (search_querie(c, &mut facts.clone(), rules), true));
                    //change facts with new value and lock value
                }
            }
        } else if c == '=' {
            break;
        }
    }
    eval_formula(rules_to_bool_str(querie_rules[0].clone(), facts).as_str())
}

fn rules_to_bool_str(rule: String, facts: &mut HashMap<char, (bool, bool)>) -> String {
    let mut result = String::new();
    for c in rule.chars() {
        if c.is_alphabetic() == true {
            if let Some((value, _)) = facts.get_mut(&c) {
                match *value == true {
                    true => result.push('1'),
                    false => result.push('0')
                }
            }
        } else if c == '&' || c == '|' || c == '^' || c == '!' {
                result.push(c);
        } else if c == '=' {
            break;
        }
    }
    result
}
