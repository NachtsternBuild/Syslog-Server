use std::fs; // filesystem
use directories::UserDirs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt; // unix perms (chmod like)
use crate::helper::write_file::write_file;

// function that set permission
// TODO: perms (755 e.g.) from cli
pub fn script_permission(filename: &str, content: &str, base_path: Option<PathBuf>, path_list: &[&str]) -> std::io::Result<()> {
	
	let create_file = write_file(&filename, &content, base_path.clone(), path_list);
	match create_file {
		Ok(_) => {
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
			// set permissions to 755
			let mut perms = fs::metadata(&full_path).unwrap().permissions();
			perms.set_mode(0o755);
			fs::set_permissions(&full_path, perms).unwrap();
			println!("[OK] Desktop Skript erstellt.");
		}
		Err(e) => eprintln!("[ERROR] Schreiben fehlgeschlagen: {}", e),
	}
	Ok(())
}
