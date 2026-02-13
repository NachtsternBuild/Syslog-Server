use std::io::{self}; // Für Terminal IO
use crate::helper::write_file::write_file;
use crate::helper::system::server_ip::server_ip;
use crate::helper::run_command::run_cmd;
use std::path::PathBuf;

// function that create the rsyslog.conf for the clients
pub fn config_client() {
	let mut content = r#"# /etc/rsyslog.conf configuration file for rsyslog
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
#module(load="imtcp")
#input(type="imtcp" port="514")

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

#
# Include all config files in /etc/rsyslog.d/
#
$IncludeConfig /etc/rsyslog.d/*.conf

###############
#### RULES ####
###############
#
#
# Log anything besides private authentification messages to a single log file
#
*.*			-/var/log/syslog # include login events

# 
# Log commonly used facilities to their own log file
#
auth,authpriv.*			/var/log/auth.log
cron.*					-/var/log/cron.log
kern.*					-/var/log/kern.log
mail.*					-/var/log/mail.log
user.*					-/var/log/user.log

#
# Emergencies are sent to everybody logged in.
#
*.emerg					:omusrmsg:*"#.to_string();
	
	// get the server ip
	match server_ip() {
		Ok(ip) => {
			content.push_str(&format!("\n*.* @@{}:514", ip)); // add the ip at the end
			println!("[?] Soll die Rsyslog Client Konfiguration direkt angwendet werden? (j/n)");
			let mut ans = String::new();
			io::stdin().read_line(&mut ans).unwrap();
			
			// add the config to /etc
			if ans.trim().to_lowercase() == "j" {
				let create_file = write_file("rsyslog.conf", &content, Some(PathBuf::from("/")), &["etc"]);
				match create_file {
					Ok(p) => println!("[OK] Datei erstellt unter: {:?}", p),
					Err(e) => eprintln!("[ERROR] Fehler: {}", e),
				}		
				println!("{}", content);
				run_cmd("systemctl", &["restart", "rsyslog"]);
			}
			// add the conf ~/client-config
			else {
				let create_file = write_file("rsyslog.conf", &content, None, &["client-config"]);
				match create_file {
					Ok(p) => println!("[OK] Datei erstellt unter: {:?}", p),
					Err(e) => eprintln!("[ERROR] Fehler: {}", e),
				}
				println!("{}", content);
			}			
		}
		Err(_e) => {
			eprintln!("[ERROR] Ein Fehler ist aufgetreten und wir sind uns nicht sicher wo dieser liegt.");
			std::process::exit(1);
		}
	}
}
