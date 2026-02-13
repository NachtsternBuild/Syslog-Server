use std::io::{self, Write}; // Für Terminal IO

mod helper;
mod utils;

use crate::{
	helper::{
		run_command::run_cmd,
		timer::timer,
		config::config_client::config_client,
		system::{
			basic_commands::basic_commands,
			system_helper::{
				refresh_system,
				cleanup,
				status_syslog_tools,
			},	
		},
	},
	utils::{
		menu::{
			config_server::config_server,
			desktop_install_menu::desktop_install_menu,
			get_rsyslog_config::get_rsyslog_config,
			firewall_menu::firewall_menu,
			change_boot_menu::change_boot_menu,
			add_log_tools::add_log_tools,
		},
	},
};

// function to check if programm run as root
fn ensure_root() {
	if unsafe { libc::geteuid() } != 0 {
		eprintln!("[ERROR] Dieses Programm benötigt Root-Rechte.");
        eprintln!("[INFO] Starte es mit sudo oder verwende den Client.");
        std::process::exit(1);
    }
}

// main
// TODO: sudo weglassen → Programm muss mit root-rechten gestartet werden
// TODO: Kommentare hinzufügen
// TODO: systemd service einrichten bei login status rsyslogd auszuzeigen
// TODO: write_file überall nutzen
// TODO: cronjob anpassungen zeit
fn main() {
	ensure_root(); // active at release
	loop {
		println!("\nWas soll gemacht werden?");
		println!("-------------------------------------");
        println!("(k) Server konfigurieren"); 
        println!("(l) Client Konfiguration ausgeben"); 
        println!("-------------------------------------");
		println!("(u) Updates und Upgrades");
        println!("(c) Nach Updates Aufräumen");
        println!("(n) Neustarten");
        println!("(i) Kommandoübersicht"); 
        println!("-------------------------------------");
        println!("(d) Desktop hinzu installieren");
        println!("(t) Zusätzliche Log Tools"); 
        println!("(r) Neue Rsyslog Config nutzen");
        println!("(f) Firewall-Modus ändern");
        println!("(s) Status von System Tools ausgeben");
        println!("(b) Boot-Modus ändern");
        println!("-------------------------------------");
        println!("(v) Verlassen/Beenden");
		
		// Show "Eingabe" now
        print!("\nEingabe: ");
        io::stdout().flush().unwrap(); 
        
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase(); // remove linebreak
        
        // switch/case 
        match choice.as_str() {
        	"k" => config_server(),
        	"l" => config_client(), 
        	"u" => refresh_system(),
        	"c" => cleanup(),
        	"n" => {
        		timer(15);
        		let args: &[&str] = &[];
        		run_cmd("reboot", args);
        	}
        	"i" => basic_commands(), 
        	"d" => desktop_install_menu(),
        	"t" => add_log_tools(), 
        	"r" => get_rsyslog_config(),
        	"f" => firewall_menu(),
        	"s" => status_syslog_tools(),
        	"b" => change_boot_menu(),
        	"v" => break, // close loop
        	_ => {
        		println!("[ERROR] Unbekannte Eingabe!");
        	}     
        }
    }
}
