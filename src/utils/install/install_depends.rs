use crate::helper::run_command::run_cmd;
use crate::helper::timer::timer;
use crate::helper::system_helper::refresh_system;
use crate::helper::config_desktop::config_desktop;
use crate::helper::system_helper::config_boot;

// Installiert die notwendigen Pakete und optionale Desktopumgebung
pub fn install_depends(with_desktop: bool) {
	refresh_system();
	let mut packages = vec!["openssh-client", "openssh-server", "rsyslog"];
	
	
	if with_desktop {
		packages.extend(["ubuntu-desktop", "gparted", "synaptic"]);
		config_desktop();
		config_boot("terminal");
	}
	
	
	let mut args = vec!["apt", "install"];
	args.extend(packages);
	run_cmd("sudo", &args);
	
	timer(10);
	run_cmd("sudo", &["reboot"]);
}   
