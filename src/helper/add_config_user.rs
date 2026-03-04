use std::io::{self, Write};
use toml::Value;
use crate::helper::read_config::write_config;

// function that allow the user to edit the config file
pub fn add_config_user() {
	let mut key_input = String::new();
	let mut value_input = String::new();
	
	// input the key
	println!("[?] Geben Sie den Namen der Einstellung ein (Key): ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut key_input).expect("[ERROR] Fehler beim Lesen");
	let key = key_input.trim();
	
	// read the data to the key
	println!("[?] Geben Sie den Wert ein: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut value_input).expect("[ERROR] Fehler beim Lesen");
	let raw_value = value_input.trim();
	
	// check the type of the data
	// check bool
	let detected_value = if let Ok(b) = raw_value.parse::<bool>() {
		Value::Boolean(b)
	}
	// check int
	else if let Ok(i) = raw_value.parse::<i64>() {
		Value::Integer(i)
	}
	
	// check float
	else if let Ok(f) = raw_value.parse::<f64>() {
		Value::Float(f)
	}
	
	// use string as default option
	else {
		Value::String(raw_value.to_string())
	};
	
	// write the data and the key
	match write_config(key, detected_value) {
		Ok(_) => println!("[OK] '{}' wurde erfolgreich gespeichert.", key),
		Err(e) => eprintln!("[ERROR] Fehler beim Speichern: {}", e),
	}
}
