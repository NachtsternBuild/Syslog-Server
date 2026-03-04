use std::path::PathBuf;
use directories::UserDirs;
use crate::helper::run_command::run_cmd;
use crate::helper::handle_user_interaction::handle_user_interaction;

// function that create the full logrotate test command
pub fn cmd_logrotate(filename: &str, base_path: Option<PathBuf>, path_list: &[&str]) -> std::io::Result<()> {
	// create base path
	let mut full_path = match base_path {
		Some(path) => path,
		None => {
			let user_dirs = UserDirs::new()
				.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "[ERROR] Home Verzeichnis nicht gefunden"))?;
			PathBuf::from(user_dirs.home_dir())
		}
	};
	
	// add subdirs
	for direc in path_list {
		full_path.push(direc);
	}
	
	// add filename
	full_path.push(filename);
	
	// create the command 
	let path_str = full_path.to_str()
		.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "[ERROR] Pfad enthält ungültige Zeichen."))?;
	let args = vec!["-d", path_str];
	run_cmd("logrotate", &args);
	Ok(())
}

// function that run a command to test the logrotate
pub fn test_logrotate() {
	let default_path = handle_user_interaction(
		"test_log_rotate_default",
		"[?] Vorgegebenen Pfad nutzen? (j/n): "
	);
	
	let input_path;
	// use default path
	if default_path.to_lowercase() == "j" {
		input_path = "/var/log/remote".to_string();
	}
	
	// use user custom path
	else {
		input_path = handle_user_interaction(
			"test_log_rotate",
			"[?] Pfad eingeben, welcher getestet werden soll: "
		);	
	}
	
	// check if path is empty	
	if input_path.is_empty() {
		println!("[INFO] Vorgang abgebrochen. Keine Änderung vorgenommen.");
		return;
	}
	
    // split the full path
    let all_elements: Vec<&str> = input_path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    // split the last element 
    // split last return tupel
    if let Some((filename, path_list)) = all_elements.split_last() {      
        let input_filename: &str = filename;
        let input_path_list: &[&str] = path_list;
        
        // run the test command
        let run_test_logrotate = cmd_logrotate(input_filename, Some(PathBuf::from("/")), input_path_list);
        match run_test_logrotate {
        	Ok(_) => println!("[OK] Kommando erfolgreich ausgeführt."),
        	Err(e) => eprintln!("[ERROR] Fehler beim Ausführen von: {}", e),
        }
    }
}
