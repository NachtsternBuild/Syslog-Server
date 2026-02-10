use std::process::{Command}; // externe programme starten

// system befehle ausführen
// I = Sammlung der Argumente 
// S = die einzelnen Argumente
pub fn run_cmd<I, S>(cmd: &str, args: I) -> bool 
where
	I: IntoIterator<Item = S>, // akzeptieren jedes I (I besteht aus S)
	S: AsRef<std::ffi::OsStr> // Alles nehmen was das OS nimmt
{
	let status = Command::new(cmd)
		.args(args)
		.status()
		.expect("[ERROR] Fehler beim Ausführen des Kommandos.");
		
	status.success()
}
