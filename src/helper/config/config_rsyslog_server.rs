use crate::helper::write_file::write_file;
use crate::helper::run_command::run_cmd;
use std::path::PathBuf;

pub fn config_rsyslog_server() {
	let content = r#"# /etc/rsyslog.conf configuration file for rsyslog
#
# For more information install rsyslog-doc and see
# /usr/share/doc/rsyslog-doc/html/configuration/index.html
#
# Default logging rules can be found in /etc/rsyslog.d/50-default.conf

#################
#### MODULES ####
#################
module(load="imuxsock") # provides support for local system logging
#module(load="immark")  # provides --MARK-- message capability

# provides UDP syslog reception
#module(load="imudp")
#input(type="imudp" port="514")

# provides TCP syslog reception
module(load="imtcp")
input(type="imtcp" port="514")

# provides kernel logging support and enable non-kernel klog messages
module(load="imklog" permitnonkernelfacility="on")

###########################
#### GLOBAL DIRECTIVES ####
###########################

# Filter duplicated messages
$RepeatedMsgReduction on

#
# Set the default permissions for all log files.
#
$FileOwner syslog
$FileGroup adm
$FileCreateMode 0640
$DirCreateMode 0755
$Umask 0022
$PrivDropToUser syslog
$PrivDropToGroup syslog

#
# Where to place spool and state files
#
$WorkDirectory /var/spool/rsyslog

########################
#### REMOTE LOGGING ####
########################
# Allowed senders - later
#$AllowedSender TCP, IP-Addresses

# Do not recevie logs with a priority higher then 4
if ($syslogpriority > 4) then {
	stop
}

# Template for remote logs
#$template RemoteLogs,"/var/log/remote/%HOSTNAME%/%fromhost-ip%/%PROGRAMNAME%.log
$template RemoteLogs,"/var/log/remote/%HOSTNAME%/%PROGRAMNAME%.log

####################################
#### INCLUDE ADDITIONAL CONFIGS ####
####################################
#
# Include all config files in /etc/rsyslog.d/
#
$IncludeConfig /etc/rsyslog.d/*.conf

# Write all remote logs using the template
# Add at the end, otherwise no further configurations will be applied
*.* ?RemoteLogs
& stop"#.to_string();

	let create_file = write_file("rsyslog.conf", &content, Some(PathBuf::from("/")), &["etc"]);
	match create_file {
		Ok(p) => println!("[OK] Datei erstellt unter: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),	
	}
	run_cmd("systemctl", &["restart", "rsyslog"]);
}
	
