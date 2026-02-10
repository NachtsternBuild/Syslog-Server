# Syslog-Server
- **A small automation project to set up a syslog server, which also serves as a playground for Rust.**

## Build
1. Install Depends:
```sh
sudo apt install cargo
sudo snap install --classic rustup
rustup install stable
cargo install cargo-deb
```
2. Build the package:
```sh
cargo run # for debugging
cargo build --release # for tests like a real release
cargo deb # build the DEB package
``` 
