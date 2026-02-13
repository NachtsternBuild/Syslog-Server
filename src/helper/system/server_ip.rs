use local_ip_address::local_ip;
use std::net::IpAddr;

// function that get the ip address from the server
pub fn server_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
	Ok(local_ip()?)
}
