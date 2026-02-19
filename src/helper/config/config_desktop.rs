use std::path::PathBuf;
use crate::helper::system::script_permission::script_permission;

// function that create a easy way enter the desktop
pub fn config_desktop() {
	let script_content = r#"#!/bin/bash
if [ -n "$DISPLAY" ] || [ -n "$WAYLAND_DISPLAY" ]; then
    echo "[ERROR] A graphical session is already running."
    exit 1
fi
echo "[INFO] Launching GNOME Wayland session..."
XDG_SESSION_TYPE=wayland dbus-run-session gnome-session"#;
	// create script
	let create_file = script_permission("start-gnome", &script_content, Some(PathBuf::from("/")), &["usr", "local", "bin"]);
	match create_file {
		Ok(p) => println!("[OK] Datei erstellt unter: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}	
}
