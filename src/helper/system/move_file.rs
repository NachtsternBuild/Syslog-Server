use std::fs;
use crate::helper::run_command::run_cmd;
use crate::helper::system::create_parent_dir::create_parent_dir;

// function that move files
pub fn move_file(path: &str, target_path: &str) {
	if path.is_empty() {
		println!("[INFO] Vorgang abgebrochen. Keine Änderung vorgenommen.");
		return;
	}
	
	if target_path.is_empty() {
		panic!("[ERROR] Vorgang abgebrochen. Der Zielpfad ist Leer.");
	}
	
	// Elternpfad prüfen
	create_parent_dir(target_path);
	
	if fs::metadata(path).is_ok() {
		if run_cmd("sudo", &["mv", path, target_path]) {
			println!("[OK] Konfiguration aktualisiert.");
		}
	}
	else {
		println!("[ERROR] Datei nicht gefunden.");
	}
}
