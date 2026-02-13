use std::thread; // threads (sleep)
use std::time::Duration; // timer for the countdown

// Countdown in the output
pub fn timer(seconds: u32) {
	for i in (1..=seconds).rev() {
		println!("[INFO] {} Sekunden verbleibend...", i);
		thread::sleep(Duration::from_secs(1));
	}
}
