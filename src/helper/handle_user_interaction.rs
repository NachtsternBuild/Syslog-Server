use std::io::{self, Write};
use crate::helper::read_config::read_config;

// function to handle action from config file or from user interaction
pub fn handle_user_interaction(config_key: &str, question: &str) -> String {
    // get the the auto_mode from the config file
    let auto_mode = read_config("auto_mode")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if auto_mode {
        // search for the key in the config file
        read_config(config_key)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Auto-Default".to_string())
    } 
    else {
        // use user action instead of preconfig from the config file
        print!("{}", question);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        input.trim().to_string()
    }
}


