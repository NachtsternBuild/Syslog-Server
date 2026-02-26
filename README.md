# Syslog-Server
- **A small automation project to set up a syslog server, which also serves as a playground for Rust.**

## Build
1. Install Depends:
```sh
sudo apt install cargo gcc-mingw-w64-x86-64 mingw-w64
sudo snap install --classic rustup
rustup install stable
rustup target add x86_64-pc-windows-gnu
cargo install cargo-deb
```
2. Build the package:
```sh
cargo run # for debugging
cargo build --release # for tests like a real release
cargo deb # build the DEB package
cargo build --target=x86_64-pc-windows-gnu --release # build for windows cli
``` 
