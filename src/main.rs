use std::io::{self, Write}; // Für Terminal IO

mod helper;
mod utils;

use crate::{
	helper::{
		run_command::run_cmd,
		timer::timer,
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
		},
		client::{
			linux_cli::linux_cli,
			free_cache_ram_cronjob::free_cache_ram_cronjob,
		},
	},
};

// main
// TODO: Docs
// TODO: Konfiguration für Client/Server aus Config datei auslesen 
// TODO: RAM/Swap/Cache freigeben → Funktion + Cronjob
// FIXME: Konfiguration Server/Client was passieren soll bei keinem Server
// FIXME: Virenschutz?

fn main() {
	//ensure_root(); // active at release
	loop {
		println!("\nWas soll gemacht werden?");
		println!("-------------------------------------");
        println!("(k) Server konfigurieren"); // TODO
        println!("(o) Client Konfiguration ausgeben"); 
        println!("-------------------------------------");
		println!("(l) Linux Client konfigurieren"); // TODO
        println!("-------------------------------------");
		println!("(u) Updates und Upgrades");
        println!("(c) Nach Updates Aufräumen");
        println!("(n) Neustarten");
        println!("(i) Kommandoübersicht"); 
        println!("-------------------------------------");
        println!("(d) Desktop hinzu installieren");
        println!("(p) Rsyslog Übersicht beim Login");
        println!("(t) Zusätzliche Log Tools"); 
        println!("(r) Neue Rsyslog Config nutzen");
        println!("(f) Firewall-Modus ändern");
        println!("(s) Status von System Tools ausgeben");
        println!("(b) Boot-Modus ändern");
        println!("(h) Cache/RAM Freigabe");
        println!("(a) Cache/RAM Freigabe als Cronjob");
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
        	"k" => config_server(),
        	"o" => config_client(), 
        	"l" => linux_cli(),
        	"u" => refresh_system(),
        	"c" => cleanup(),
        	"n" => {
        		timer(15);
        		let args: &[&str] = &[];
        		run_cmd("reboot", args);
        	}
        	"i" => basic_commands(), 
        	"d" => desktop_install_menu(),
        	"p" => config_pam_rsyslog(),
        	"t" => add_log_tools(), 
        	"r" => get_rsyslog_config(),
        	"f" => firewall_menu(),
        	"s" => status_syslog_tools(),
        	"b" => change_boot_menu(),
        	"h" => {
        		match free_cache_ram() {
        			Ok(_) => println!("[OK] Speicher wurde freigegeben."),
        			Err(e) => eprintln!("[ERROR] Fehler bei Freigeben des Caches/RAMs: {}", e),
        		}
        	}
        	"a" => free_cache_ram_cronjob(),
        	"v" => break, // close loop
        	_ => {
        		println!("[ERROR] Unbekannte Eingabe!");
        	}     
        }
    }
}
