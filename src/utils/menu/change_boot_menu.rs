use std::io::{self}; // Für Terminal IO
use crate::helper::system_helper::config_boot;

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
