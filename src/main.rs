mod algo;
mod operator;
mod parsing;

use std::collections::{HashMap, HashSet};
use parsing::{fill_maps, Variable};
use algo::algo_v1;

// struct Rule {
//     input: fn(&HashMap<char, (Variable, Vec<Rule>)>) -> bool,
//     output: BTree, // A <=> B  [A, {...A}, {input A, output B}] [B, {...B}, {input B, output A}]
//     formula_string: String
// }

fn main() -> Result<(), String> {
    let file = "test_files/frog";
    //let mut variables: HashMap<char, (Variable, Vec<Rule>)> = HashMap::new();
    // let mut rules = HashSet::new();
    // fill_maps(&mut variables, &mut rules,  file)?;
    // print_vars(&variables);
    algo_v1();
    Ok(())
}

fn print_vars(variables: &HashMap<String, Variable>)  {
    println!("name v|l|r|true|false");
    for (k, v) in variables.iter() {
        println!("{k} -> {v}")
    }
}