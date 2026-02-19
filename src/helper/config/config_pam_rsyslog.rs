use crate::helper::add_line_file::add_line_file;

// function that config the pam settings
pub fn config_pam_rsyslog() {
	let files = ["/etc/pam.d/login", "/etc/pam.d/sshd"];
	let pam_line = "session optional pam_exec.so stdout /bin/systemctl status rsyslog.service";
	for file in files {
		let pam_config = add_line_file(&true, file, pam_line);
		match pam_config {
			Ok(p) => println!("[OK] Pam Konfiguration erstellt: {:?}", p),
			Err(e) => eprintln!("[ERROR] Fehler: {}", e),
		}
		if let Err(e) = add_line_file(&true, file, pam_line) {
			eprintln!("[ERROR] Fehler bei {}: {}", file, e);
		}
	}
}
