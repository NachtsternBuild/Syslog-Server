use crate::{
	utils::{
		client::free_cache_ram_cronjob::free_cache_ram_cronjob,
		menu::{
			desktop_install_menu::desktop_install_menu,
			add_log_tools::add_log_tools,
			change_boot_menu::change_boot_menu,
			firewall_menu::firewall_menu,
			logrotate::logrotate,
		},
	},
	helper::config::{
		config_desktop::config_desktop,
		config_rsyslog_server::config_rsyslog_server,
		config_client::config_client,
		config_pam_rsyslog::config_pam_rsyslog,
	},
};
	
// a main config script
pub fn config_server() {
	desktop_install_menu();
	add_log_tools();
	config_desktop();
	change_boot_menu();
	firewall_menu();
	config_rsyslog_server();
	config_client();
	free_cache_ram_cronjob();
	logrotate();
	config_pam_rsyslog();
}
