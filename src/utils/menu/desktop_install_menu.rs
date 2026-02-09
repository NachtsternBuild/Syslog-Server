use std::io::{self}; // Für Terminal IO
use crate::utils::install::install_depends::install_depends;

pub fn desktop_install_menu() {
	println!("[?] Desktop installieren? (j/n): ");
	let mut ans = String::new();
	io::stdin().read_line(&mut ans).unwrap();
	install_depends(ans.trim().to_lowercase() == "j");
}
