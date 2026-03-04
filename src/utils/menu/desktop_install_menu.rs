use crate::utils::install::install_depends::install_depends;
use crate::helper::handle_user_interaction::handle_user_interaction;

// function that install desktop by user choice
pub fn desktop_install_menu() {
	let ans = handle_user_interaction(
		"desktop_install_menu",
		"[?] Desktop installieren? (j/n): "
	);
	
	install_depends(ans.trim().to_lowercase() == "j"); // install depends
}
