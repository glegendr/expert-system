mod algo;
mod models;
mod parsing;
mod leakser;
mod utils;
mod interactive;
mod translate;

use std::collections::HashMap;
use parsing::{fill_maps};
use models::Variable;
use algo::algo_v1;
use leakser::{leaks, Flag};
use utils::print_variables;
use translate::Lang;
use interactive::interactive_mode;

fn main() -> Result<(), String> {
    let (files, mut flags) = leaks()?;
    let lang = match flags.iter().find(|flag| match flag {Flag::Lang(_) => true, _ => false}).unwrap_or(&Flag::Lang(Lang::En)) {
        Flag::Lang(l) => l,
        _ => unreachable!()
    };
    if flags.iter().any(|f| f == &Flag::Interactive) {
        interactive_mode(&files, &mut flags);
    } else {
        for file in files.iter() {
            let mut variables: HashMap<char, Variable> = HashMap::new();
            fill_maps(&mut variables, file, true)?;
            if flags.contains(&Flag::Variables) {
                print_variables(&variables);
            }
            algo_v1(&mut variables, flags.contains(&Flag::Trace), &lang);
        }
    }
    Ok(())
}