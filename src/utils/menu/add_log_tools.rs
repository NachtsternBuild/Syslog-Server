use crate::helper::config::config_cleanup_log::config_cleanup_log;
use crate::helper::config::config_cleanup_log::config_cleanup_log_files;
use crate::helper::config::config_basic_cmd::config_basic_cmd;

// function that run all log tools
pub fn add_log_tools() {
	config_cleanup_log_files();
	config_cleanup_log();
	config_basic_cmd();
}
