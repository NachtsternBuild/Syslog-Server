use std::fs;
use std::path::PathBuf;
use directories::UserDirs;
use crate::helper::run_command::run_cmd;

// function that write a file 
// None = Home as Base
// Some = define a other base
pub fn move_file(
	filename: &str, 
	base_path: Option<PathBuf>, 
	path_list: &[&str], 
	target_filename: &str, 
	base_path_target: Option<PathBuf>, 
	target_path_list: &[&str]) -> std::io::Result<()> {
	// create base path
	let mut full_path = match base_path {
		Some(path) => path,
		None => {
			let user_dirs = UserDirs::new()
				.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "[ERROR] Home Verzeichnis nicht gefunden"))?;
			PathBuf::from(user_dirs.home_dir())
		}
	};
	
	// create base path
	let mut full_path_target = match base_path_target {
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
	
	for target_direc in target_path_list {
		full_path_target.push(target_direc);
	}
	
	// add filename
	full_path.push(filename);
	full_path_target.push(target_filename);
	
	// create path structure
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent)?;
	}
	
	if let Some(parent_target) = full_path_target.parent() {
		fs::create_dir_all(parent_target)?;
	}
		
	if fs::metadata(&full_path).is_ok() {
		let full_path_str = full_path.to_str().ok_or_else(|| {
			std::io::Error::new(std::io::ErrorKind::InvalidInput, "[ERROR] Ungültiger Dateipfad")
		})?;
		
    	let full_path_target_str = full_path_target.to_str().ok_or_else(|| {
        	std::io::Error::new(std::io::ErrorKind::InvalidInput, "[ERROR] Ungültiger Zielpfad")
    	})?;

    	if run_cmd("sudo", &["mv", full_path_str, full_path_target_str]) {
        	println!("[OK] Konfiguration aktualisiert.");
    	}
	} else {
    	println!("[ERROR] Datei nicht gefunden.");
	}
	
	Ok(())
}
