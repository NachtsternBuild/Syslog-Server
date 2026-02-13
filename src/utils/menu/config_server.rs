use crate::{
	utils::menu::{
		desktop_install_menu::desktop_install_menu,
		add_log_tools::add_log_tools,
		change_boot_menu::change_boot_menu,
		firewall_menu::firewall_menu,
	},
	helper::config::{
		config_desktop::config_desktop,
		config_rsyslog_server::config_rsyslog_server,
		config_client::config_client,
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
}
