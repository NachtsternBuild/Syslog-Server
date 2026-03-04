use std::fs;
use std::path::Path;
use toml::Value;

// default path to the settings.toml
//const CONFIG_PATH: &str = "settings.toml";
// for deb release
const CONFIG_PATH: &str = "/etc/syslog-server/settings.toml";

// function to read the config files 
pub fn read_config(key: &str) -> Option<Value> {
	// check if file exists
	if !Path::new(CONFIG_PATH).exists() {
		eprintln!("[ERROR] Fehler: {} fehlt.", CONFIG_PATH);
		return None;
	}
	
	// read and parse the toml file
	let content = fs::read_to_string(CONFIG_PATH).ok()?;
	let table = content.parse::<Value>().ok()?;
	
	// return the key
	table.get(key).cloned()
}

// function that write the configs to a file
pub fn write_config(key: &str, new_value: Value) -> Result<(), String> {
	// read the full file and create a empty table if no exists
	let mut table = if Path::new(CONFIG_PATH).exists() {
		let content = fs::read_to_string(CONFIG_PATH)
			.map_err(|e| format!("[ERROR] Fehler beim Lesen: {}", e))?;
		content.parse::<Value>()
			.map_err(|e| format!("[ERROR] Fehler beim Parsen: {}", e))?
	}
	else {
		Value::Table(toml::map::Map::new())
	};
	
	// set the value in the memory (update or insert)
	if let Some(table_map) = table.as_table_mut() {
		table_map.insert(key.to_string(), new_value);
	}
	else  {
		return Err("[ERROR] Die Konfigurationsdatei ist keine gültige TOML-Tabelle".to_string());
	}
	
	// convert to toml string
	let toml_string = toml::to_string_pretty(&table)
		.map_err(|e| format!("[ERROR] Fehler beim Serialisieren: {}", e))?;
	
	// write the file
	fs::write(CONFIG_PATH, toml_string)
		.map_err(|e| format!("[ERROR] Fehler beim Schreiben der Datei: {}", e))?;
	
	Ok(())
}


