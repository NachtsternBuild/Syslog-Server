use crate::helper::print_cmd::print_cmd;

// show basic ubuntu commands
pub fn basic_commands() {
	println!("===================== SYSTEM INFO =====================\n");
	print_cmd("hostnamectl", "Show system hostname and OS info", "hostnamectl");
	print_cmd("uname -a", "Display kernel information", "uname -a");
	print_cmd("uptime", "Show how long the server has been running", "uptime");
	print_cmd("date", "Display current date/time", "date");
	print_cmd("timedatectl", "Manage system time", "timedatectl set-timezone Europe/Berlin");
	print_cmd("arch", "Show system architecture", "arch");
	print_cmd("lscpu", "CPU details", "lscpu");
	print_cmd("lsblk", "List block devices", "lsblk");
	print_cmd("free -h", "Show memory usage", "free -h");
	print_cmd("df -h", "Disk usage overview", "df -h");

	println!("\n\n===================== PACKAGE MANAGEMENT =====================\n");
	print_cmd("apt update", "Refresh package index", "sudo apt update");
	print_cmd("apt upgrade", "Upgrade installed packages", "sudo apt upgrade");
	print_cmd("apt full-upgrade", "Handle dependency changes", "sudo apt full-upgrade");
	print_cmd("apt install", "Install a package", "sudo apt install nginx");
	print_cmd("apt remove", "Remove package", "sudo apt remove nginx");
	print_cmd("apt purge", "Remove package + config", "sudo apt purge nginx");
	print_cmd("apt autoremove", "Remove unused dependencies", "sudo apt autoremove");
	print_cmd("apt search", "Search for package", "apt search docker");
	print_cmd("apt show", "Package details", "apt show openssh-server");
	print_cmd("dpkg -l", "List installed packages", "dpkg -l | less");

	println!("\n\n===================== USER MANAGEMENT =====================\n");
	print_cmd("adduser", "Create a new user", "sudo adduser devops");
	print_cmd("userdel", "Delete a user", "sudo userdel devops");
	print_cmd("usermod -aG", "Add user to group", "sudo usermod -aG sudo devops");
	print_cmd("passwd", "Change password", "sudo passwd devops");
	print_cmd("id", "Show user identity", "id devops");
	print_cmd("who", "Show logged-in users", "who");
	print_cmd("w", "Show active sessions", "w");
	print_cmd("last", "Login history", "last");
	print_cmd("groups", "Display user groups", "groups devops");
	print_cmd("chage -l", "Password aging info", "sudo chage -l devops");

	println!("\n\n===================== FILE MANAGEMENT =====================\n");
	print_cmd("ls -lah", "Detailed directory listing", "ls -lah /var/log");
	print_cmd("cd", "Change directory", "cd /etc");
	print_cmd("pwd", "Show current directory", "pwd");
	print_cmd("cp", "Copy files", "cp file.txt /backup/");
	print_cmd("mv", "Move or rename", "mv old.txt new.txt");
	print_cmd("rm -rf", "Delete files/directories", "rm -rf temp/");
	print_cmd("touch", "Create empty file", "touch test.txt");
	print_cmd("mkdir -p", "Create directories", "mkdir -p /data/backups");
	print_cmd("find", "Search files", "find / -name nginx.conf");
	print_cmd("locate", "Fast file search", "locate sshd_config");

	println!("\n\n===================== PERMISSIONS =====================\n");
	print_cmd("chmod", "Change file permissions", "chmod 640 secret.txt");
	print_cmd("chown", "Change file owner", "chown user:user file.txt");
	print_cmd("umask", "Default permission mask", "umask 027");
	print_cmd("getfacl", "View ACL", "getfacl file.txt");
	print_cmd("setfacl", "Set ACL", "setfacl -m u:devops:r file.txt");
	
	println!("\n\n===================== PROCESS MANAGEMENT =====================\n");
	print_cmd("ps aux", "List processes", "ps aux | grep nginx");
	print_cmd("top", "Real-time process viewer", "top");
	print_cmd("htop", "Enhanced process viewer", "htop");
	print_cmd("kill", "Terminate process", "kill 1234");
	print_cmd("killall", "Kill by name", "killall nginx");
	print_cmd("nice", "Start with priority", "nice -n 10 backup.sh");
	print_cmd("renice", "Change priority", "renice 5 -p 1234");
	print_cmd("pgrep", "Find process IDs", "pgrep ssh");
	print_cmd("pkill", "Kill via pattern", "pkill apache2");
	print_cmd("bg / fg", "Background/foreground jobs", "fg %1");

	println!("\n\n===================== SERVICES (SYSTEMD) =====================\n");
	print_cmd("systemctl status", "Service status", "systemctl status ssh");
	print_cmd("systemctl start", "Start service", "sudo systemctl start nginx");
	print_cmd("systemctl stop", "Stop service", "sudo systemctl stop nginx");
	print_cmd("systemctl restart", "Restart service", "sudo systemctl restart nginx");
	print_cmd("systemctl reload", "Reload config", "sudo systemctl reload nginx");
	print_cmd("systemctl enable", "Enable at boot", "sudo systemctl enable nginx");
	print_cmd("systemctl disable", "Disable at boot", "sudo systemctl disable nginx");
	print_cmd("systemctl list-units", "List services", "systemctl list-units --type=service");
	print_cmd("journalctl -xe", "View logs", "journalctl -xe");
	print_cmd("journalctl -u", "Logs for service", "journalctl -u ssh");

	println!("\n\n===================== NETWORK =====================\n");
	print_cmd("ip a", "Show IP addresses", "ip a");
	print_cmd("ip r", "Routing table", "ip r");
	print_cmd("ss -tulpen", "Listening ports", "ss -tulpen");
	print_cmd("ping", "Test connectivity", "ping -c 4 google.com");
	print_cmd("curl", "Fetch HTTP data", "curl https://example.com");
	print_cmd("wget", "Download files", "wget https://example.com/file.iso");
	print_cmd("nmcli", "NetworkManager CLI", "nmcli con show");
	print_cmd("netplan apply", "Apply network config", "sudo netplan apply");
	print_cmd("dig", "DNS lookup", "dig openai.com");
	print_cmd("traceroute", "Trace network path", "traceroute google.com");

	println!("\n\n===================== STORAGE =====================\n");
	print_cmd("mount", "Mount filesystem", "mount /dev/sdb1 /mnt");
	print_cmd("umount", "Unmount filesystem", "umount /mnt");
	print_cmd("blkid", "Show UUIDs", "blkid");
	print_cmd("fsck", "Check filesystem", "fsck /dev/sdb1");
	print_cmd("du -sh", "Directory size", "du -sh /var/log");
	print_cmd("fdisk -l", "Partition table", "sudo fdisk -l");
	print_cmd("parted -l", "Advanced partition info", "sudo parted -l");
	print_cmd("mkfs.ext4", "Create filesystem", "mkfs.ext4 /dev/sdb1");
	print_cmd("rsync", "Efficient file sync", "rsync -av /data /backup");
	print_cmd("tar", "Archive files", "tar -czvf backup.tar.gz /data");

	println!("\n\n===================== SECURITY =====================\n");
	print_cmd("ufw status", "Firewall status", "sudo ufw status");
	print_cmd("ufw allow", "Allow port", "sudo ufw allow 22");
	print_cmd("ufw enable", "Enable firewall", "sudo ufw enable");
	print_cmd("fail2ban-client status", "Fail2ban overview", "sudo fail2ban-client status");
	print_cmd("ssh-keygen", "Generate SSH keys", "ssh-keygen -t ed25519");
	print_cmd("chmod 600", "Secure SSH keys", "chmod 600 ~/.ssh/id_ed25519");
	print_cmd("visudo", "Edit sudoers safely", "sudo visudo");
	print_cmd("lastlog", "Check last logins", "lastlog");
	print_cmd("dmesg", "Kernel messages", "dmesg | less");
	print_cmd("apparmor_status", "AppArmor status", "sudo apparmor_status");

	println!("\n\n===================== LOGS & MONITORING =====================\n");
	print_cmd("tail -f", "Follow log file", "tail -f /var/log/syslog");
	print_cmd("less", "Read large files", "less /var/log/syslog");
	print_cmd("watch", "Run command repeatedly", "watch -n 1 free -h");
	print_cmd("uptime", "System load", "uptime");
	print_cmd("vmstat", "Memory stats", "vmstat 1");
	print_cmd("iostat", "Disk IO stats", "iostat");
	print_cmd("sar", "Historical metrics", "sar -u 1 3");
	print_cmd("ncdu", "Disk usage analyzer", "ncdu /");
	print_cmd("lsof", "Open files", "lsof -i :22");
	print_cmd("strace", "Trace syscalls", "strace -p 1234");
}
