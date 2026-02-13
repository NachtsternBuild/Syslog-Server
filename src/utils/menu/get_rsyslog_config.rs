use std::fs; // Für Dateisystem-Operationen 
use std::io::{self}; // Für Terminal IO
use crate::helper::run_command::run_cmd;

// verschiebt eine datei nach /etc/rsyslog.conf
pub fn get_rsyslog_config() {
	println!("[?] Pfad zur neuen rsyslog.conf: ");
	let mut path = String::new();
	io::stdin().read_line(&mut path).unwrap();
	let path = path.trim();
	
	if path.is_empty() {
		println!("[INFO] Vorgang abgebrochen. Keine Änderung vorgenommen.");
		return;
	}
	
	if fs::metadata(path).is_ok() {
		if run_cmd("mv", &[path, "/etc/rsyslog.conf"]) {
			println!("[OK] Konfiguration aktualisiert.");
		}
	}
	else {
		println!("[ERROR] Datei nicht gefunden.");
	}
}
