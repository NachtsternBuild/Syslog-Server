use std::io::{self};
use crate::helper::run_command::run_cmd;
use crate::helper::timer::timer;
use crate::helper::system::system_helper::refresh_system;
use crate::helper::config::config_desktop::config_desktop;
use crate::helper::config::config_boot::config_boot;

// install depends and opt. desktop env
pub fn install_depends(with_desktop: bool) {
	// refresh the system
	refresh_system();
	let mut packages = vec!["openssh-client", "openssh-server", "rsyslog"]; // default depends
	
	// with desktop
	if with_desktop {
		packages.extend(["ubuntu-desktop", "gparted", "synaptic"]);
		config_desktop(); // config desktop env
		config_boot("terminal"); // config boot mode
	}
	
	let mut args = vec!["install"];
	args.extend(packages);
	run_cmd("apt", &args);
	
	println!("[?] System jetzt neustarten? (j/n)");
	let mut ans = String::new();
	io::stdin().read_line(&mut ans).unwrap();
	if ans.trim().to_lowercase() == "j" {
		println!("[INFO] System wird heruntergefahren...");
		timer(15); // start timer
		let args: &[&str] = &[];
		run_cmd("reboot", args);
	}
	else {
		println!("[INFO] Führen Sie sobald wie möglich einen Neustart durch.");
	}
}   
