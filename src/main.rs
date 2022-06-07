mod algo;
mod models;
mod parsing;
mod leakser;
mod utils;
mod interactive;

use std::collections::HashMap;
use parsing::{fill_maps};
use models::Variable;
use algo::algo_v1;
use leakser::{leaks, Flag};
use utils::print_variables;
use interactive::interactive_mode;

fn main() -> Result<(), String> {
    let (files, flags) = leaks()?;
    if flags.iter().any(|f| f == &Flag::Interactive) {
        interactive_mode(&files, &flags);
    } else {
        for file in files.iter() {
            let mut variables: HashMap<char, Variable> = HashMap::new();
            fill_maps(&mut variables, file, true)?;
            print_variables(&variables);
            algo_v1(&mut variables);
        }
    }
    Ok(())
}
