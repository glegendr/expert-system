mod algo;
mod operator;
mod parsing;
mod leakser;

use std::collections::{HashMap, HashSet};
use parsing::{fill_maps, Variable, Rule};
use algo::algo_v1;
use leakser::leaks;

// struct Rule {
//     input: fn(&HashMap<char, (Variable, Vec<Rule>)>) -> bool,
//     output: BTree, // A <=> B  [A, {...A}, {input A, output B}] [B, {...B}, {input B, output A}]
//     formula_string: String
// }

fn main() -> Result<(), String> {
    let files = leaks()?;
    for file in files.iter() {
        let mut variables = HashMap::new();
        // let mut rules = HashSet::new();
        fill_maps(&mut variables, file)?;
        // print_vars(&variables);
        algo_v1();
    }
    Ok(())
}

fn print_vars(variables: &HashMap<String, Variable>)  {
    println!("name v|l|r|true|false");
    for (k, v) in variables.iter() {
        println!("{k} -> {v}")
    }
}

fn print_rules(rules: &HashSet<Rule>)  {
    for r in rules.iter() {
        println!("{r}")
    }
}