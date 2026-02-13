use std::path::Path;
use std::fs;

// function to create parent dir
pub fn create_parent_dir(path: &str) {
	let target_path = Path::new(path);
    
    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("[ERROR] Ordner konnte nicht erstellt werden");
        }
    }
}
