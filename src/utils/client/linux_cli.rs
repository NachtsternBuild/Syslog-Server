use std::io;
use std::net::IpAddr;
use std::path::PathBuf;
use crate::helper::write_file::write_file;
use crate::helper::add_line_file::add_line_file;

pub fn linux_cli() {
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
	println!("[?] Geben Sie die Server IP ein: ");
	let create_file = write_file("rsyslog.conf", &content, Some(PathBuf::from("/")), &["etc"]);
	match create_file {
		Ok(p) => println!("[OK] Datei geschrieben: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}

	let mut ans = String::new();
	io::stdin().read_line(&mut ans).unwrap();
	let address = ans.trim();
	let path;
	match address.parse::<IpAddr>() {
		Ok(_) => {
			path = format!("*.* @@{}:514", address);
		}
		Err(_) => {
			println!("[WARN] Eingabe ist keine IP-Addresse.");
			return;
		}
	}
		
	let file = "/etc/rsyslog.conf";
	let l_cli_config = add_line_file(&true, &file, &path);
	match l_cli_config {
		Ok(p) => println!("[OK] Linux Client Konfiguration erstellt: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}
	if let Err(e) = add_line_file(&true, &file, &path) {
		eprintln!("[ERROR] Fehler bei {:?}: {}", file, e);
	}

}
