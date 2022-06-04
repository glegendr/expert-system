use std::collections::HashMap;
use crate::models::eval_formula;

pub fn algo_v1() {
    let mut rules: Vec<String> = Vec::new();
    rules.push(String::from("A B! & => F"));
    rules.push(String::from("B A! & => C"));
    rules.push(String::from("A B & => M"));
    rules.push(String::from("F M | => G"));
    rules.push(String::from("C => Y"));
    rules.push(String::from("U V | => Z"));
    rules.push(String::from("A B & => Z"));
    let mut facts = HashMap::new();
    facts.insert('A',(true, true));
    facts.insert('B',(true, true));
    facts.insert('C',(false, false));
    facts.insert('F',(false, false));
    facts.insert('M',(false, false));
    facts.insert('G',(false, false));
    facts.insert('Y',(false, false));
    facts.insert('U',(false, false));
    facts.insert('V',(false, false));
    facts.insert('Z',(false, false));
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
    //TOUP -> check all seconds parts of RULES & keep RULES with querie in second part
    let mut querie_rules = Vec::new();
    for rule in rules.iter() {
        if rule.chars().last() == Some(querie) {
            querie_rules.push(rule);
        }
    }
    if querie_rules.len() == 0 {
        facts.insert(querie, (false, true));
        return false
    }
    //TODO -> do all rule until true
    let mut i = 0;
    let mut all_fact_is_true = false;
    while i < querie_rules.len() {
        for c in querie_rules[i].chars() {
            if c.is_alphabetic() == true {
                if let Some((_, lock)) = facts.get_mut(&c) {
                    if *lock == false {
                        match search_querie(c, &mut facts.clone(), rules) {
                            true => {
                                facts.insert(c, (true, true));
                                all_fact_is_true = true;
                            }
                            _ => {
                                all_fact_is_true = false;
                                continue
                            }
                        };
                    } else {
                        all_fact_is_true = true;
                    }
                }
            } else if c == '=' {
                break;
            }
        }
        if all_fact_is_true == true {
            break;
        }
        i += 1;
    }
    if all_fact_is_true == false && i == querie_rules.len() {
        facts.insert(querie, (false, true));
        return false
    }
    match eval_formula(rules_to_bool_str(querie_rules[i].clone(), facts).as_str()) {
        true => {
            facts.insert(querie, (true, true));
            true
        },
        _ => {
            facts.insert(querie, (false, true));
            false
        }
    }
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
        } else if "&|^!".contains(c) {
                result.push(c);
        } else if c == '=' {
            break;
        }
    }
    result
}
