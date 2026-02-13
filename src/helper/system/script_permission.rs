use std::fs; // filesystem
use std::os::unix::fs::PermissionsExt; // unix perms (chmod like)
use crate::helper::system::create_parent_dir::create_parent_dir;

// function that set permission
// TODO: perms (755 e.g.) from cli
pub fn script_permission(path: &str, script_content: &str) {
	// Elternpfad überprüfen
	create_parent_dir(path);
	
	match fs::write(path, script_content) {
		Ok(_) => {
			// setzen der Datei Berechtigungen auf 755 (chmod a+x)
			let mut perms = fs::metadata(path).unwrap().permissions();
			perms.set_mode(0o755);
			fs::set_permissions(path, perms).unwrap();
			println!("[OK] Desktop Skript erstellt.");
		}
		Err(e) => eprintln!("[ERROR] Schreiben fehlgeschlagen: {}", e),
	}
}
