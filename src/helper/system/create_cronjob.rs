use crate::helper::run_command::run_cmd;

// function that create a cronjob
// TODO: time by cli
pub fn create_cronjob(path: &str) {
	println!("1. Tippen Sie im Terminal '1' um nano zu öffnen");
	println!("2. Ergänzen Sie am Ende der Datei folgende Zeile:");
	println!("0 2 * * {}", path);
	run_cmd("crontab", &["-e"]);
}
