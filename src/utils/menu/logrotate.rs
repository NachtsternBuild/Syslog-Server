use crate::helper::config::config_logrotate::config_logrotate;
use crate::helper::system::test_logrotate::test_logrotate;
use crate::helper::handle_user_interaction::handle_user_interaction;

// function that were you can choose with logrotate task you want
pub fn logrotate() {
	let run_config_logrotate = handle_user_interaction(
		"logrotate_config",
		"[?] Soll Logrotate konfiguriert werden? (j/n): "
	);
	
	let run_test_logrotate = handle_user_interaction(
		"logrotate_test",
		"[?] Soll Logrotate getestet werden? (j/n): "
	);
	
	if run_config_logrotate.to_lowercase() == "j" {
		let configure = config_logrotate();
		match configure {
			Ok(_) => println!("[OK] Logrotate erfolgreich konfiguriert."),
			Err(_) => eprintln!("[ERROR] Fehler beim Konfigurieren von Logrotate"),
		}
	}
	
	if run_test_logrotate.to_lowercase() == "j" {
		test_logrotate();
	}
}
