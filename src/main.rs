mod algo;
mod models;
mod parsing;
mod leakser;
mod utils;

use std::collections::HashMap;
use parsing::fill_maps;
use models::Variable;
use algo::algo_v1;
use leakser::leaks;
use utils::print_variables;

fn main() -> Result<(), String> {
    let files = leaks()?;
    for file in files.iter() {
        let mut variables: HashMap<char, Variable> = HashMap::new();
        fill_maps(&mut variables, file)?;
        print_variables(&variables);
        algo_v1();
    }
    Ok(())
}