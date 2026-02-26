use crate::helper::system::create_cronjob::create_cronjob_shell;

pub fn free_cache_ram_cronjob() {
	let free1 = create_cronjob_shell("0 0 * * *", "sync; echo 1 > /proc/sys/vm/drop_caches");
	match free1 {
		Ok(p) => println!("[OK] Cronjob erstellt: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}
	let free2 = create_cronjob_shell("0 0 * * *", "sync; echo 2 > /proc/sys/vm/drop_caches");
	match free2 {
		Ok(p) => println!("[OK] Cronjob erstellt: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}
	let free3 = create_cronjob_shell("0 0 * * *", "sync; echo 3 > /proc/sys/vm/drop_caches");
	match free3 {
		Ok(p) => println!("[OK] Cronjob erstellt: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}
	let free4 = create_cronjob_shell("0 0 * * *", "swapoff -a && swapon -a");
	match free4 {
		Ok(p) => println!("[OK] Cronjob erstellt: {:?}", p),
		Err(e) => eprintln!("[ERROR] Fehler: {}", e),
	}
}
