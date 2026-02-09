use std::thread; // threads (sleep)
use std::time::Duration; // zeit für timer

// Countdown im Terminal
pub fn timer(seconds: u32) {
	for i in (1..=seconds).rev() {
		println!("[INFO] {} Sekunden verbleibend...", i);
		thread::sleep(Duration::from_secs(1));
	}
}
