use std::io::{self, Write}; // Für Terminal IO

mod helper;
mod utils;

use crate::{
	helper::{
		run_command::run_cmd,
		timer::timer,
		add_config_user::add_config_user,
		config::{
			config_client::config_client,
			config_pam_rsyslog::config_pam_rsyslog,
		},
		system::{
			basic_commands::basic_commands,
			free_cache_ram::free_cache_ram,
			system_helper::{
				refresh_system,
				cleanup,
				status_syslog_tools,
				ensure_root,
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
			logrotate::logrotate,
		},
		client::{
			linux_cli::linux_cli,
			free_cache_ram_cronjob::free_cache_ram_cronjob,
		},
	},
};

// main
// TODO: Docs
// FIXME: Server konfiguration schreiben ?

fn main() {
	//ensure_root(); // active at release
	loop {
		println!("\nWas soll gemacht werden?");
		println!("-------------------------------------");
        println!("(a) Server konfigurieren"); // TODO
        println!("(b) Client Konfiguration ausgeben"); 
        println!("-------------------------------------");
		println!("(c) Linux Client konfigurieren"); 
        println!("-------------------------------------");
		println!("(d) Updates und Upgrades");
        println!("(e) Nach Updates Aufräumen");
        println!("(f) Neustarten");
        println!("(g) Kommandoübersicht"); 
        println!("-------------------------------------");
        println!("(h) Desktop hinzu installieren");
        println!("(i) Rsyslog Übersicht beim Login");
        println!("(j) Zusätzliche Log Tools"); 
        println!("(k) Neue Rsyslog Config nutzen");
        println!("(l) Firewall-Modus ändern");
        println!("(m) Status von System Tools ausgeben");
        println!("(n) Boot-Modus ändern");
        println!("(o) Cache/RAM Freigabe");
        println!("(p) Cache/RAM Freigabe als Cronjob");
        println!("(q) Tool Konfiguration ändern");
        println!("(r) Logrotation einrichten/testen"); 
        println!("-------------------------------------");
        println!("(v) Verlassen/Beenden");
		
		// Show "Eingabe" now
        print!("\n[?] Eingabe: ");
        io::stdout().flush().unwrap(); 
        
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase(); // remove linebreak
        
        // switch/case 
        match choice.as_str() {
        	"a" => config_server(),
        	"b" => config_client(), 
        	"c" => linux_cli(),
        	"d" => refresh_system(),
        	"e" => cleanup(),
        	"f" => {
        		timer(15);
        		let args: &[&str] = &[];
        		run_cmd("reboot", args);
        	}
        	"g" => basic_commands(), 
        	"h" => desktop_install_menu(),
        	"i" => config_pam_rsyslog(),
        	"j" => add_log_tools(), 
        	"k" => get_rsyslog_config(),
        	"l" => firewall_menu(),
        	"m" => status_syslog_tools(),
        	"n" => change_boot_menu(),
        	"o" => {
        		match free_cache_ram() {
        			Ok(_) => println!("[OK] Speicher wurde freigegeben."),
        			Err(e) => eprintln!("[ERROR] Fehler bei Freigeben des Caches/RAMs: {}", e),
        		}
        	}
        	"p" => free_cache_ram_cronjob(),
        	"q" => add_config_user(),
        	"r" => logrotate(),
        	"v" => break, // close loop
        	_ => {
        		println!("[ERROR] Unbekannte Eingabe!");
        	}     
        }
    }
}
