use crate::helper::run_command::run_cmd;
use crate::helper::handle_user_interaction::handle_user_interaction;

// config the firewall
pub fn firewall_menu() {
	let ans = handle_user_interaction(
		"firewall_menu_config",
		"[?] Modus: (a) Alles erlauben / (s) SSH & Port 514: "
	);
	
	run_cmd("ufw", &["enable"]);
	if ans.trim() == "s" {
		run_cmd("ufw", &["default", "deny"]);
        run_cmd("ufw", &["allow", "ssh"]);
        run_cmd("ufw", &["allow", "514/tcp"]);
    }
    else {
    	run_cmd("ufw", &["default", "allow"]);
    }
    
    run_cmd("ufw", &["delogging", "on"]);
    run_cmd("ufw", &["reload"]);
}
