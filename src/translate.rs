use colored::Colorize;

#[derive(PartialEq, Clone)]
pub enum Lang {
    En,
    Fr,
    It
}

pub enum Translate {
    WeKnow,
    WeAlreadyKnow,
    NoRule,
    Rule,
    And,
    So,
    Help,
    HelpLanguage,
    HelpTrace,
    HelpReset,
    HelpQuit,
    HelpVariables,
    HelpRules,
    HelpClear,
    HelpFile,
    HelpRun,
    HelpRemoveAll,
    HelpRemoveVar,
    HelpRemoveRule,
    HelpRemoveRequest,
    HelpRemoveSet,
    HelpSet,
    HelpRequest,
    HelpDef,
    HelpIf,
    UnknownCommand,
}

impl Translate {
    pub fn print<T: std::fmt::Display>(&self, lang: &Lang, opt_1: T, opt_2: Option<bool>) {
        match opt_2 {
            Some(opt_2) => {
                match lang {
                    Lang::En => {
                        let status = match opt_2 {
                            true => "true".green(),
                            false => "false".red()
                        };
                        match self {
                            // Trace
                            Translate::WeKnow => println!("We known {} is {} because", opt_1, status),
                            Translate::WeAlreadyKnow => println!("We already know that {} is {}", opt_1, status),
                            Translate::NoRule => println!("We know {} is {} because no rule assign it.", opt_1, status),
                            Translate::Rule => print!("We have{}. ", opt_1),
                            Translate::And => print!("and {} is {} ", opt_1, status),
                            Translate::So => print!("so {} is {} ", opt_1, status),
                            // Help
                            Translate::Help => println!("{}\n - display all commands or asked one", opt_1),
                            Translate::HelpLanguage => println!("{}\n - change default language to chosen one", opt_1),
                            Translate::HelpTrace => println!("{}\n - unable/disable algorithm's trace", opt_1),
                            Translate::HelpReset => println!("{}\n - clear the map and reload all providen files", opt_1),
                            Translate::HelpQuit => println!("{}\n - quit the program", opt_1),
                            Translate::HelpVariables => println!("{}\n - list all variables and their rules", opt_1),
                            Translate::HelpRules => println!("{}\n - list all rules", opt_1),
                            Translate::HelpClear => println!("{}\n - alias for \"remove all\"", opt_1),
                            Translate::HelpFile => println!("{}\n - read the file in path and enrich variables and rules", opt_1),
                            Translate::HelpRun => println!("{}\n - run the algorithm with variable if providen", opt_1),
                            Translate::HelpRemoveAll => println!("{}\n - clear all variables and rules", opt_1),
                            Translate::HelpRemoveVar => println!("{}\n - remove the variable and all rules implicated", opt_1),
                            Translate::HelpRemoveRule => println!("{}\n - remove the rule depending the index listed with \"rules\"", opt_1),
                            Translate::HelpRemoveRequest => println!("{}\n - remove the variable from requested one", opt_1),
                            Translate::HelpRemoveSet => println!("{}\n - remove the variable from seted one", opt_1),
                            Translate::HelpSet => println!("{}\n - set the variable(s) to true", opt_1),
                            Translate::HelpRequest => println!("{}\n - set the variable(s) to requested", opt_1),
                            Translate::HelpDef => println!("{}\n - create a new variable", opt_1),
                            Translate::HelpIf => println!("{}\n - create a new rule", opt_1),
                            Translate::UnknownCommand => println!("unknown command {}", opt_1)
                        }
                    }
                    Lang::Fr => {
                        let status = match opt_2 {
                            true => "vrai".green(),
                            false => "faux".red()
                        };
                        match self {
                            // Trace
                            Translate::WeKnow => println!("Nous savons que {} est {} car", opt_1, status),
                            Translate::WeAlreadyKnow => println!("Nous savons déjà que {} est {}", opt_1, status),
                            Translate::NoRule => println!("Nous savons que {} est {} car aucune règle ne l'assigne.", opt_1, status),
                            Translate::Rule => print!("Nous avons{}. ", opt_1),
                            Translate::And => print!("et {} est {} ", opt_1, status),
                            Translate::So => print!("donc {} est {} ", opt_1, status),
                            // Help
                            Translate::Help => println!("{}\n - affiche toutes les commandes ou celles demmandées", opt_1),
                            Translate::HelpLanguage => println!("{}\n - change la langue par celle choisie", opt_1),
                            Translate::HelpTrace => println!("{}\n - active/désactive la trace de l'alorithme", opt_1),
                            Translate::HelpReset => println!("{}\n - vide les donnés et ré-importe les fichier", opt_1),
                            Translate::HelpQuit => println!("{}\n - quitte le programme", opt_1),
                            Translate::HelpVariables => println!("{}\n - liste toutes les variables et leur règles", opt_1),
                            Translate::HelpRules => println!("{}\n - liste toutes les règles", opt_1),
                            Translate::HelpClear => println!("{}\n - alias pour \"remove all\"", opt_1),
                            Translate::HelpFile => println!("{}\n - importe le fichier et enrichie les donnés", opt_1),
                            Translate::HelpRun => println!("{}\n - lance l'algorithme avec les variables si données", opt_1),
                            Translate::HelpRemoveAll => println!("{}\n - vide les variables et les règles", opt_1),
                            Translate::HelpRemoveVar => println!("{}\n - supprime la variable et toutes les règles l'impliquant", opt_1),
                            Translate::HelpRemoveRule => println!("{}\n - supprime la règle selon l'index donné avec \"rules\"", opt_1),
                            Translate::HelpRemoveRequest => println!("{}\n - supprime la variable des requêtes", opt_1),
                            Translate::HelpRemoveSet => println!("{}\n - met la variable a faux", opt_1),
                            Translate::HelpSet => println!("{}\n - met la variable(s) a vrai", opt_1),
                            Translate::HelpRequest => println!("{}\n - ajoute la variable(s) aux requêtes", opt_1),
                            Translate::HelpDef => println!("{}\n - créer une nouvelle variable", opt_1),
                            Translate::HelpIf => println!("{}\n - créer une nouvelle règle", opt_1),
                            Translate::UnknownCommand => println!("commande inconnue {}", opt_1)
                        }
                    }
                    Lang::It => {
                        let status = match opt_2 {
                            true => "vero".green(),
                            false => "falso".red()
                        };
                        match self {
                            // Trace
                            Translate::WeKnow => println!("Sappiamo che {} è {} perché", opt_1, status),
                            Translate::WeAlreadyKnow => println!("Sappiamo già che {} è {}", opt_1, status),
                            Translate::NoRule => println!("Sappiamo che {} è {} perché nessuna regola lo assegna.", opt_1, status),
                            Translate::Rule => print!("Noi abbiamo{}. ", opt_1),
                            Translate::And => print!("e {} è {} ", opt_1, status),
                            Translate::So => print!("così {} è {} ", opt_1, status),
                            // Help
                            Translate::Help => println!("{}\n - visualizza tutti i comandi o ne viene chiesto uno", opt_1),
                            Translate::HelpLanguage => println!("{}\n - cambia lingua in base a quella prescelta", opt_1),
                            Translate::HelpTrace => println!("{}\n - incapace/disabilita la traccia dell'algoritmo", opt_1),
                            Translate::HelpReset => println!("{}\n - cancellare i dati e ricaricare tutti i file forniti", opt_1),
                            Translate::HelpQuit => println!("{}\n - uscire dal programma", opt_1),
                            Translate::HelpVariables => println!("{}\n - elenca tutte le variabili e le relative regole", opt_1),
                            Translate::HelpRules => println!("{}\n - elenca tutte le regole", opt_1),
                            Translate::HelpClear => println!("{}\n - alias per \"remove all\"", opt_1),
                            Translate::HelpFile => println!("{}\n - leggi il file nel percorso e arricchisci variabili e regole", opt_1),
                            Translate::HelpRun => println!("{}\n - eseguire l'algoritmo con la variabile se fornita", opt_1),
                            Translate::HelpRemoveAll => println!("{}\n - cancellare tutte le variabili e le regole", opt_1),
                            Translate::HelpRemoveVar => println!("{}\n - rimuovere la variabile e tutte le regole implicate", opt_1),
                            Translate::HelpRemoveRule => println!("{}\n - rimuovere la regola a seconda dell'indice elencato con \"rules\"", opt_1),
                            Translate::HelpRemoveRequest => println!("{}\n - rimuovere la variabile da quella richiesta", opt_1),
                            Translate::HelpRemoveSet => println!("{}\n - rimuovere la variabile da quella impostata", opt_1),
                            Translate::HelpSet => println!("{}\n - imposta la variabil.e.i su true", opt_1),
                            Translate::HelpRequest => println!("{}\n - impostare la variabil.e.i su richiesta", opt_1),
                            Translate::HelpDef => println!("{}\n - creare una nuova variabile", opt_1),
                            Translate::HelpIf => println!("{}\n - creare una nuova regola", opt_1),
                            Translate::UnknownCommand => println!("comando sconosciuto {}", opt_1)
                        }
                    }
                }
            },
            None => {
                match lang {
                    Lang::En => {
                        match self {
                            // Trace
                            Translate::WeKnow => println!("We known {} because", opt_1),
                            Translate::WeAlreadyKnow => println!("We already know that {}", opt_1),
                            Translate::NoRule => println!("We know {} because no rule assign it.", opt_1),
                            Translate::Rule => print!("We have{}. ", opt_1),
                            Translate::And => print!("and {} ", opt_1),
                            Translate::So => print!("so {} ", opt_1),
                            // Help
                            Translate::Help => println!("{}\n - display all commands or asked one", opt_1),
                            Translate::HelpLanguage => println!("{}\n - change default language to chosen one", opt_1),
                            Translate::HelpTrace => println!("{}\n - unable/disable algorithm's trace", opt_1),
                            Translate::HelpReset => println!("{}\n - clear the map and reload all providen files", opt_1),
                            Translate::HelpQuit => println!("{}\n - quit the program", opt_1),
                            Translate::HelpVariables => println!("{}\n - list all variables and their rules", opt_1),
                            Translate::HelpRules => println!("{}\n - list all rules", opt_1),
                            Translate::HelpClear => println!("{}\n - alias for \"remove all\"", opt_1),
                            Translate::HelpFile => println!("{}\n - read the file in path and enrich variables and rules", opt_1),
                            Translate::HelpRun => println!("{}\n - run the algorithm with variable if providen", opt_1),
                            Translate::HelpRemoveAll => println!("{}\n - clear all variables and rules", opt_1),
                            Translate::HelpRemoveVar => println!("{}\n - remove the variable and all rules implicated", opt_1),
                            Translate::HelpRemoveRule => println!("{}\n - remove the rule depending the index listed with \"rules\"", opt_1),
                            Translate::HelpRemoveRequest => println!("{}\n - remove the variable from requested one", opt_1),
                            Translate::HelpRemoveSet => println!("{}\n - remove the variable from seted one", opt_1),
                            Translate::HelpSet => println!("{}\n - set the variable(s) to true", opt_1),
                            Translate::HelpRequest => println!("{}\n - set the variable(s) to requested", opt_1),
                            Translate::HelpDef => println!("{}\n - create a new variable", opt_1),
                            Translate::HelpIf => println!("{}\n - create a new rule", opt_1),
                            Translate::UnknownCommand => println!("unknown command {}", opt_1)
                        }
                    }
                    Lang::Fr => {
                        match self {
                            // Trace
                            Translate::WeKnow => println!("Nous savons que {} car", opt_1),
                            Translate::WeAlreadyKnow => println!("Nous savons déjà que {}", opt_1),
                            Translate::NoRule => println!("Nous savons que {} car aucune règle ne l'assigne.", opt_1),
                            Translate::Rule => print!("Nous avons{}. ", opt_1),
                            Translate::And => print!("et {} ", opt_1),
                            Translate::So => print!("donc {} ", opt_1),
                            // Help
                            Translate::Help => println!("{}\n - affiche toutes les commandes ou celles demmandées", opt_1),
                            Translate::HelpLanguage => println!("{}\n - change la langue par celle choisie", opt_1),
                            Translate::HelpTrace => println!("{}\n - active/désactive la trace de l'alorithme", opt_1),
                            Translate::HelpReset => println!("{}\n - vide les donnés et ré-importe les fichier", opt_1),
                            Translate::HelpQuit => println!("{}\n - quitte le programme", opt_1),
                            Translate::HelpVariables => println!("{}\n - liste toutes les variables et leur règles", opt_1),
                            Translate::HelpRules => println!("{}\n - liste toutes les règles", opt_1),
                            Translate::HelpClear => println!("{}\n - alias pour \"remove all\"", opt_1),
                            Translate::HelpFile => println!("{}\n - importe le fichier et enrichie les donnés", opt_1),
                            Translate::HelpRun => println!("{}\n - lance l'algorithme avec les variables si données", opt_1),
                            Translate::HelpRemoveAll => println!("{}\n - vide les variables et les règles", opt_1),
                            Translate::HelpRemoveVar => println!("{}\n - supprime la variable et toutes les règles l'impliquant", opt_1),
                            Translate::HelpRemoveRule => println!("{}\n - supprime la règle selon l'index donné avec \"rules\"", opt_1),
                            Translate::HelpRemoveRequest => println!("{}\n - supprime la variable des requêtes", opt_1),
                            Translate::HelpRemoveSet => println!("{}\n - met la variable a faux", opt_1),
                            Translate::HelpSet => println!("{}\n - met la variable(s) a vrai", opt_1),
                            Translate::HelpRequest => println!("{}\n - ajoute la variable(s) aux requêtes", opt_1),
                            Translate::HelpDef => println!("{}\n - créer une nouvelle variable", opt_1),
                            Translate::HelpIf => println!("{}\n - créer une nouvelle règle", opt_1),
                            Translate::UnknownCommand => println!("commande inconnue {}", opt_1)
                        }
                    }
                    Lang::It => {
                        match self {
                            // Trace
                            Translate::WeKnow => println!("Sappiamo che {} perché", opt_1),
                            Translate::WeAlreadyKnow => println!("Sappiamo già che {}", opt_1),
                            Translate::NoRule => println!("Sappiamo che {} perché nessuna regola lo assegna.", opt_1),
                            Translate::Rule => print!("Noi abbiamo{}. ", opt_1),
                            Translate::And => print!("e {} ", opt_1),
                            Translate::So => print!("così {} ", opt_1),
                            // Help
                            Translate::Help => println!("{}\n - visualizza tutti i comandi o ne viene chiesto uno", opt_1),
                            Translate::HelpLanguage => println!("{}\n - cambia lingua in base a quella prescelta", opt_1),
                            Translate::HelpTrace => println!("{}\n - incapace/disabilita la traccia dell'algoritmo", opt_1),
                            Translate::HelpReset => println!("{}\n - cancellare i dati e ricaricare tutti i file forniti", opt_1),
                            Translate::HelpQuit => println!("{}\n - uscire dal programma", opt_1),
                            Translate::HelpVariables => println!("{}\n - elenca tutte le variabili e le relative regole", opt_1),
                            Translate::HelpRules => println!("{}\n - elenca tutte le regole", opt_1),
                            Translate::HelpClear => println!("{}\n - alias per \"remove all\"", opt_1),
                            Translate::HelpFile => println!("{}\n - leggi il file nel percorso e arricchisci variabili e regole", opt_1),
                            Translate::HelpRun => println!("{}\n - eseguire l'algoritmo con la variabile se fornita", opt_1),
                            Translate::HelpRemoveAll => println!("{}\n - cancellare tutte le variabili e le regole", opt_1),
                            Translate::HelpRemoveVar => println!("{}\n - rimuovere la variabile e tutte le regole implicate", opt_1),
                            Translate::HelpRemoveRule => println!("{}\n - rimuovere la regola a seconda dell'indice elencato con \"rules\"", opt_1),
                            Translate::HelpRemoveRequest => println!("{}\n - rimuovere la variabile da quella richiesta", opt_1),
                            Translate::HelpRemoveSet => println!("{}\n - rimuovere la variabile da quella impostata", opt_1),
                            Translate::HelpSet => println!("{}\n - imposta la variabil.e.i su true", opt_1),
                            Translate::HelpRequest => println!("{}\n - impostare la variabil.e.i su richiesta", opt_1),
                            Translate::HelpDef => println!("{}\n - creare una nuova variabile", opt_1),
                            Translate::HelpIf => println!("{}\n - creare una nuova regola", opt_1),
                            Translate::UnknownCommand => println!("comando sconosciuto {}", opt_1)
                        }
                    }
                }
            }
        }
    }
}