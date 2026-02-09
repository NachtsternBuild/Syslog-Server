use std::fs; // Für Dateisystem-Operationen 
use std::os::unix::fs::PermissionsExt; // UNix spezifisch (chmod)

pub fn config_desktop() {
	let script_content = r#"#!/bin/bash
if [ -n "$DISPLAY" ] || [ -n "$WAYLAND_DISPLAY" ]; then
    echo "[ERROR] A graphical session is already running."
    exit 1
fi
echo "[INFO] Launching GNOME Wayland session..."
XDG_SESSION_TYPE=wayland dbus-run-session gnome-session"#;
	
	let path = "/usr/local/bin/start-gnome";
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
