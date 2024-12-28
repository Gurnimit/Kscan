# Kscan

Kscan is a Rust-based scanning tool designed to help you efficiently scan either by ip , hostname or both . 

## Features
- Efficient scanning
- Easy to use
- Built with Rust

## Installation

1. **Clone the Repository:**
   '''
   `git clone https://github.com/Gurnimit/Kscan.git`
   cd Kscan
   '''
3. **Additional Installation**
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustup update stable
   sudo chown -R $(whoami) /home/<your device name>/Kscan
   sudo chmod -R u+rwX /home/<your device name>/Kscan
   cargo clean
   cargo build --release
4. **Way to Use**
   ~ cargo run -- --hostname <hostname> --ports <rang of ports(00-00)>
   ~ cargo run -- --ip <ip> --ports <rang of ports(00-00)>
   ~cargo run -- --ip <ip> --hostname <hostname> --ports <rang of ports(00-00)>
