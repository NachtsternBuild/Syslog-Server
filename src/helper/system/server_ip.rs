use local_ip_address::local_ip;
use std::net::IpAddr;

pub fn server_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
	Ok(local_ip()?)
}
