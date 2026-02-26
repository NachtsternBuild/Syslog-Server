use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::error::Error;

// function that write at the cache
fn write_drop_cache(value: &str) -> Result<(), Box<dyn Error>> {
	let mut file = OpenOptions::new()
		.write(true)
		.open("/proc/sys/vm/drop_caches")?;
		
	file.write_all(value.as_bytes())?;
	file.flush()?; // get safe that everything is written 
	Ok(()) 
}

// function that free the cache now
pub fn free_cache_ram() -> Result<(), Box<dyn Error>> {
	// free filesystem buffer
	unsafe {
		libc::sync()
	};
	
	// freeing caches (3 = Pagecache + Dentries + Inodes)
	write_drop_cache("3")?;
	
	// freeing the swap an reactivate the swap
	let off_status = Command::new("swapoff").arg("-a").status()?;
	if !off_status.success() {
		eprintln!("[WARN] Swap konnte nicht deaktiviert werden.");
	}
	else {
		Command::new("swapon").arg("-a").status()?;
	}
	Ok(())
}
