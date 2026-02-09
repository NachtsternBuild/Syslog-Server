use std::io::{self, Write}; // Für Terminal IO

mod helper;
mod utils;

use crate::helper::run_command::run_cmd;
use crate::helper::timer::timer;
use crate::helper::system_helper::refresh_system;
use crate::helper::system_helper::cleanup;
use crate::utils::menu::config_server::config_server;
use crate::utils::menu::desktop_install_menu::desktop_install_menu;
use crate::utils::menu::get_rsyslog_config::get_rsyslog_config;
use crate::utils::menu::firewall_menu::firewall_menu;
use crate::helper::system_helper::status_syslog_tools;
use crate::utils::menu::change_boot_menu::change_boot_menu;

fn main() {
	loop {
		println!("\nWas soll gemacht werden?");
		println!("------------------------------------");
        println!("(u) Updates und Upgrades");
        println!("(c) Nach Updates Aufräumen");
        println!("(n) Neustarten");
        println!("(a) Abmelden");
        println!("-------------------------------------");
        println!("(k) Server konfigurieren");
        println!("(d) Desktop hinzu installieren");
        println!("(r) Neue Rsyslog Config nutzen");
        println!("(f) Firewall-Modus ändern");
        println!("(s) Status von System Tools ausgeben");
        println!("(b) Boot-Modus ändern");
        println!("-------------------------------------");
        println!("(v) Verlassen/Beenden");

        print!("\nEingabe: ");
        io::stdout().flush().unwrap(); // "Eingabe:" sofort anzeigen
        
        let mut input = String::new(); // String für input
        io::stdin().read_line(&mut input).unwrap(); // liest Zeile von stdin
        let choice = input.trim().to_lowercase(); // entfernt Leerzeoichen/Zeilenumbrüche
        
        // switch/case 
        match choice.as_str() {
        	"u" => refresh_system(),
        	"c" => cleanup(),
        	"k" => config_server(),
        	"d" => desktop_install_menu(),
        	"r" => get_rsyslog_config(),
        	"f" => firewall_menu(),
        	"s" => status_syslog_tools(),
        	"b" => change_boot_menu(),
        	"n" => {
        		timer(15);
        		run_cmd("sudo", &["reboot"]);
        	}
        	"a" => {
        		timer(15);
        		let args: &[&str] = &[]; // Expliziter Typ hilft dem Compiler
        		run_cmd("logout", args);
        	}
        	"v" => break, // schleife beenden
        	_ => {
        		println!("[ERROR] Unbekannte Eingabe!");
        	}     
        }
    }
}
