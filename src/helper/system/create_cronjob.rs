use std::process::Command;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use directories::UserDirs;

// function that create a cronjob with a file as target
pub fn create_cronjob(cron: &str, filename: &str, base_path: Option<PathBuf>, path_list: &[&str]) -> std::io::Result<()> {
	// create base path
	let mut full_path = match base_path {
		Some(path) => path,
		None => {
			let user_dirs = UserDirs::new()
				.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "[ERROR] Home Verzeichnis nicht gefunden"))?;
			PathBuf::from(user_dirs.home_dir())
		}
	};
	
	// add subdirs
	for direc in path_list {
		full_path.push(direc);
	}
	// add filename
	full_path.push(filename);
	
	// create full command for cron
	let mut cronjob_name = String::from(cron);
	cronjob_name.push_str(&full_path.to_string_lossy()); // convert and add
	
	// check for active cronjobs
	let mut cron_jobs = String::new();
	let output = Command::new("crontab")
		.arg("-l")
		.output()
		.expect("[ERROR] Fehler beim Abrufen der Cronjobs");
		
	if output.status.success() {
		cron_jobs = String::from_utf8_lossy(&output.stdout).to_string();
	}
	
	// check if the cronjob exists
	if !cron_jobs.contains(&cronjob_name) {
		cron_jobs.push_str(&format!("{}\n", cronjob_name));
		
		// create new cronjob
		let mut crontab = OpenOptions::new()
			.write(true)
			.truncate(true)
			.open("/tmp/crontab_temp")
			.expect("[ERROR] Fehler beim erstellen der temp Datei");
		
		writeln!(crontab, "{}", cron_jobs).expect("[ERROR] Fehler beim Schreiben in die Temporäre Cron Datei");
		
		// add the new cronjob 
		Command::new("crontab")
			.arg("/tmp/crontab_tmp")
			.output()
			.expect("[ERROR] Fehler beim setzen der Cronjobs");
		println!("[OK] Cronjob hinzugefügt: {}", cronjob_name);	
	}
	else {
		println!("[WARN] Diesen Cronjob gibt es schon");
	}
	
	// remove tmp file
	std::fs::remove_file("/tmp/crontab_temp").expect("[ERROR] Fehler beim Löschen der tmp Datei");
	
	Ok(())
}

// create cronjob without a file
pub fn create_cronjob_shell(cron_schedule: &str, command: &str) -> std::io::Result<()> {
    // get active cronjobs
    let output = Command::new("crontab")
        .arg("-l")
        .output()?;

    let mut cron_jobs = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::new()
    };

    // format the cronjob
    let new_job = format!("{} {}", cron_schedule, command);

    // check if cronjob exists
    if cron_jobs.lines().any(|line| line == new_job) {
        println!("[WARN] Dieser Cronjob existiert bereits.");
        return Ok(());
    }

    // add new cron
    cron_jobs.push_str(&format!("{}\n", new_job));

    // write a tmp file
    let temp_path = "/tmp/crontab_temp";
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(temp_path)?;
        file.write_all(cron_jobs.as_bytes())?;
    }

    // update cronjobs
    let status = Command::new("crontab")
        .arg(temp_path)
        .status()?;

    if status.success() {
        println!("[OK] Cronjob erfolgreich hinzugefügt: {}", new_job);
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "[ERROR] Crontab konnte nicht aktualisiert werden"));
    }

    // cleanup
    std::fs::remove_file("/tmp/crontab_temp").expect("[ERROR] Fehler beim Löschen der tmp Datei");

    Ok(())
}

