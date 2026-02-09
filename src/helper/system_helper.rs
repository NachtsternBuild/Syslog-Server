use crate::helper::run_command::run_cmd;

//pub mod system_helper {
// updates/upgrades
pub fn refresh_system() {
	run_cmd("sudo", &["apt", "update"]);
	run_cmd("sudo", &["apt", "upgrade"]);
	run_cmd("sudo", &["snap", "refresh"]);
}

// aufräumen
pub fn cleanup() {
	run_cmd("sudo", &["apt", "autoremove"]);
	run_cmd("sudo", &["apt", "autoclean"]);
}

// status systemtools
pub fn status_syslog_tools() {
	run_cmd("systemctl", &["status", "ssh", "--no-pager"]);
	run_cmd("systemctl", &["status", "rsyslog", "--no-pager"]);
}

// boot ziel ändern
pub fn config_boot(mode: &str) {
	let target = if mode == "grf" {
		"graphical.target"
	}
	else {
		"multi-user.target"
	};
	run_cmd("sudo", &["systemctl", "set-default", target]);
}
//}
