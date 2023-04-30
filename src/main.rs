mod rassfi;
use rassfi::Rassfi;

use anyhow::Result;
use std::{env, process::exit};

fn main() -> Result<()> {
    let valid_actions: Vec<&str> = vec!["encrypt", "login"];
    let args: Vec<String> = env::args().collect();

    let rassfi = Rassfi::new()?;
    // When no arguments are passed defaults to 'login'
    if args.len() == 1 {
        rassfi.display_accounts();
        exit(0);
    } else if args.len() > 2 {
        println!("Unexpected number of arguments. Expected only one got {} instead.", args.len() - 1);
        exit(1);
    }

    // This is guaranteed to be safe
    let action = args.get(1).unwrap().as_str();
    if !valid_actions.contains(&action) {
        println!("Expected either 'encrypt' or 'login' got {}", action);
        exit(1);
    }

    if action == "encrypt" {
        rassfi.prompt_service_name()?;
    } else if action == "login" {
        rassfi.display_accounts();
    } 

    Ok(())
}
