use std::io::{self}; 
use crate::helper::run_command::run_cmd;

// config the firewall
pub fn firewall_menu() {
	println!("[?] Modus: (a) Alles erlauben / (s) SSH & Port 514: ");
	let mut ans = String::new();
	io::stdin().read_line(&mut ans).unwrap();
	
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
