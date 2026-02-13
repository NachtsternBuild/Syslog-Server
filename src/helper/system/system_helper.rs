use crate::helper::run_command::run_cmd;

// updates/upgrades
pub fn refresh_system() {
	run_cmd("apt", &["update"]);
	run_cmd("apt", &["upgrade"]);
	run_cmd("snap", &["refresh"]);
}

// cleanup after update
pub fn cleanup() {
	run_cmd("apt", &["autoremove"]);
	run_cmd("apt", &["autoclean"]);
}

// status systemtools
// TODO: weitere tools
pub fn status_syslog_tools() {
	run_cmd("systemctl", &["status", "ssh", "--no-pager"]);
	run_cmd("systemctl", &["status", "rsyslog", "--no-pager"]);
}
