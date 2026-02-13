use std::io::{self}; // Für Terminal IO
use crate::helper::config::config_boot::config_boot;

// functiopn that switch the boot mode by user choice
pub fn change_boot_menu() {
	println!("[?] Grafisch booten? (j/n): ");
	let mut ans = String::new();
	io::stdin().read_line(&mut ans).unwrap();
	if ans.trim().to_lowercase() == "j" {
		config_boot("grf");
	}
	else {
		config_boot("terminal");
	}
}
