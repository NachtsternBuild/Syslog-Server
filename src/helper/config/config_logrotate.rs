use std::path::PathBuf;
use crate::helper::write_file::write_file;
/*
@brief: 
/var/log/remote/**/*.log {
    daily                       # daily check
    rotate 90                   # 90 days
    size 200M                   # filesize = 200M rotate
    missingok                   # no errors if file missing
    notifempty                  # ignore empty files 
    compress                    # compress (gzip)
    delaycompress               # first archiv are uncompressed
    sharedscripts               # run script only on time at the day
    postrotate
        /usr/lib/rsyslog/rsyslog-rotate
    endscript
}
*/

// function that write a logroation file to /etc/logrotate.d
pub fn config_logrotate() -> std::io::Result<()> {
	let script_content = r#"/var/log/remote/**/*.log {
	daily
	rotate 90
	size 200M
	compress
	delaycompress
	missingok
	notifempty
	sharedscripts
	postrotate
		/usr/lib/rsyslog/rsyslog-rotate
	endscript	
}"#;
	let create_file = write_file("remote-logs", &script_content, Some(PathBuf::from("/")), &["etc", "logrotate.d"]);
	match create_file {
		Ok(p) => println!("[OK] Datei erstellt unter: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}
	Ok(())	
}
