use crate::helper::config::config_boot::config_boot;
use crate::helper::handle_user_interaction::handle_user_interaction;

// functiopn that switch the boot mode by user choice
pub fn change_boot_menu() {
	let ans = handle_user_interaction(
		"change_boot_menu",
		"[?] Grafisch booten? (j/n): "
	);
	
	if ans.trim().to_lowercase() == "j" {
		config_boot("grf");
	}
	else {
		config_boot("terminal");
	}
}
