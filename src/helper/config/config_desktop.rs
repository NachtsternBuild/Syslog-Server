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
	
	let path = "/usr/local/bin/start-gnome"; // FIXME
	// create script
	script_permission(path, script_content);
}
