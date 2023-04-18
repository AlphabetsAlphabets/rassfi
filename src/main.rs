mod rassfi;
use rassfi::Rassfi;
use srsa::errors::KeyError;

use std::{env, process::exit};

fn main() -> Result<(), KeyError> {
    let valid_actions: Vec<&str> = vec!["encrypt", "login"];
    let args: Vec<String> = env::args().collect();
    let action = if let Some(action) = args.get(1) {
        action.as_str()
    } else {
        println!("One argument required. Valid arugments are 'encrypt' and 'login'.");
        exit(1)
    };

    if !valid_actions.contains(&action) {
        println!("Invalid argument '{}'. Expected 'encrypt' or 'login'.", action);
        exit(1);
    }

    let rassfi = Rassfi::new()?;
    if action == "encrypt" {
        rassfi.prompt_service_name();
    } else if action == "login" {
        rassfi.display_accounts();
    }

    Ok(())
}
