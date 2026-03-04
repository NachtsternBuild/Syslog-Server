use std::fs; // Für Dateisystem-Operationen 
use crate::helper::run_command::run_cmd;
use crate::helper::handle_user_interaction::handle_user_interaction;

// verschiebt eine datei nach /etc/rsyslog.conf
pub fn get_rsyslog_config() {
	let path = handle_user_interaction(
		"get_rsyslog_config_path",
		"[?] Pfad zur neuen rsyslog.conf: "
	);
	
	if path.is_empty() {
		println!("[INFO] Vorgang abgebrochen. Keine Änderung vorgenommen.");
		return;
	}
	
	if fs::metadata(&path).is_ok() {
		if run_cmd("mv", &[&path, "/etc/rsyslog.conf"]) {
			println!("[OK] Konfiguration aktualisiert.");
		}
	}
	else {
		println!("[ERROR] Datei nicht gefunden.");
	}
}
