use std::process::{Command}; // externe programme starten

// run system command
// I = the list of arguments 
// S = the argmeunts
pub fn run_cmd<I, S>(cmd: &str, args: I) -> bool 
where
	I: IntoIterator<Item = S>, // allow all I
	S: AsRef<std::ffi::OsStr> // allow all from OS
{
	let status = Command::new(cmd)
		.args(args)
		.status()
		.expect("[ERROR] Fehler beim Ausführen des Kommandos.");
		
	status.success()
}
