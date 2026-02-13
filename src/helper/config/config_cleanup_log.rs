use std::io::{self}; // Für Terminal IO
use crate::helper::system::script_permission::script_permission;
use crate::helper::system::move_file::move_file;
use crate::helper::system::create_cronjob::create_cronjob;

// function that create a script that cleanup old log files
pub fn config_cleanup_log_files() {
	let script_content = r#"!/bin/bash
# Skript um alte Log Dateien zu löschen
# Funktion um alte Logs zu löschen
find_old_log() {
	DIRECTORY=$1
	# Verzeicchnis exsistiert ?
	if [ ! -d "$DIRECTORY" ]; then
		echo "[ERROR] Directory $DIRECTORY not found!"
		exit 1
	}
	# Lösche alle Log-Dateien älter 90 Tage
	find "$DIRECTORY" -type f -name "*.log" -mtime +990 -exec rm -rf {} \;
}

# Main
# Datei definieren wo alle zu prüfende Verzeichnisse darin stehen
datei="/usr/local/share/logging/directory.conf"
# schleife für alle dateien
while IFS= read -r element; do
	find_old_log $element
done < "$datei"
echo \"[INFO] All log files in $datei that are older than 90 days habe been deleted!\""#;
	let path = "/usr/local/bin/cleanup-log-files";
	// create script
	script_permission(path, script_content);
	
	println!("\n\n[?] Pfad für neue directory.conf: ");
	let mut path_dir = String::new();
	io::stdin().read_line(&mut path_dir).unwrap();
	let path_dir = path_dir.trim();
	let target_path = "/usr/local/share/logging/directory.conf"; // FIXME
	
	// move file
	move_file(path_dir, target_path);
	// create cronjob
	create_cronjob(path); // FIXME: Zeit mit übergeben 
}

// script that clean the logs
pub fn config_cleanup_log() {
	let script_content = r#"!/bin/bash
# Skript um alte Log Einträge in den Log-Dateien zu löschen
# Datum bestimmen YYYY-MM-DD → vor 90 Tagen
CUT_OFF_DATE=$(date --date="90 days ago" +%Y-%m-%d)
# Funktion um Log-Dateien zu überprüfen
find_log_files() {
	$DIRECTORY=$1
	# Überprüfen, ob das Verzeichnis exsistiert
	if [ ! -d "$DIRECTORY" ]; then 
		echo "[ERROR] Directory $DIRECTORY not found!"
		exit 1
	fi
	
	# Durchlaufe alle Log-Dateien in einem Verzeichnis
	for LOG_FILE in "$DIRECTORY"/*.log; do
		if [ -f "$LOG_FILE" ]; then
			# Erstelle Temp Datei für bereignigte Log-Dateien
			TEMP_FILE=$(mktemp)
			# Verarbeite jede Zeile → löschen von zu alten Zeilen
			while IFS= read -r LINE; do
				# Extrahieren des Datums 
				LOG_DATE=$(echo "$LINE" | awk -F'T' '{print $1}')
				# Wenn Datum < CUT_OFF_DATE → write temp file
				if  [[ "$LOG_DATE" > "$CUT_OFF_DATE" ]]; then
					echo "$LINE" >> "$TEMP_FILE"
				fi
			done < "$LOG_FILE" 
			# Ersetzt Originaldatei durch bereinigte Datei
			mv "$TEMP_FILE" "$LOG_FILE"
			echo "[OK] Log file $LOG_FILE has been cleanup up."
		fi
	done
}

# Main
datei="/usr/local/share/logging/directory.conf"
# Alle Dateien durchlaufen
while IFS= read -r element; do
	fin_old_log $element
done < "$datei"
echo "[INFO] All log files in $datei directory have been deleted!""#;
	let path = "/usr/local/bin/cleanup-log";
	// create script
	script_permission(path, script_content);
	
	println!("\n\n[?] Pfad für neue directory.conf: ");
	let mut path_dir = String::new();
	io::stdin().read_line(&mut path_dir).unwrap();
	let path_dir = path_dir.trim();
	let target_path = "/usr/local/share/logging/directory.conf"; // FIXME
	
	// move file
	move_file(path_dir, target_path);
	// create cronjob
	create_cronjob(path); // FIXME
}
