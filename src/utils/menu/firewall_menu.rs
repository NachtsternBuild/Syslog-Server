use std::io::{self}; // Für Terminal IO
use crate::helper::run_command::run_cmd;

// komnfiguratzion firewall
pub fn firewall_menu() {
	println!("[?] Modus: (a) Alles erlauben / (s) SSH & Port 514: ");
	let mut ans = String::new();
	io::stdin().read_line(&mut ans).unwrap();
	
	run_cmd("sudo", &["ufw", "enable"]);
	if ans.trim() == "s" {
		run_cmd("sudo", &["ufw", "default", "deny"]);
        run_cmd("sudo", &["ufw", "allow", "ssh"]);
        run_cmd("sudo", &["ufw", "allow", "514/tcp"]);
    }
    else {
    	run_cmd("sudo", &["ufw", "default", "allow"]);
    }
    
    run_cmd("sudo", &["ufw", "delogging", "on"]);
    run_cmd("sudo", &["ufw", "reload"]);
}
