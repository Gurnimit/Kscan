# Kscan

Kscan is a Rust-based scanning tool designed to help you efficiently scan either by IP address, hostname, or both. Whether you're a network administrator, security enthusiast, or developer, Kscan offers a reliable and customizable solution for your scanning needs.

## Features <br/>
- Efficient scanning <br/>
- Easy to use <br/>
- Built with Rust <br/>

## Installation

1. **Clone the Repository:** <br/>
   `git clone https://github.com/Gurnimit/Kscan.git` <br/>
   `cd Kscan` <br/>
2. **Additional Installation** <br/>
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` <br/>
   `source $HOME/.cargo/env` <br/>
   `rustup update stable` <br/>
   `sudo chown -R $(whoami) /home/<your device name>/Kscan` <br/>
   `sudo chmod -R u+rwX /home/<your device name>/Kscan` <br/>
   `cargo clean` <br/>
   `cargo build --release` <br/>
3. **Way to Use** <br/>
   `cargo run -- --hostname <hostname> --ports <rang of ports(00-00)>` <br/>
   `cargo run -- --ip <ip> --ports <rang of ports(00-00)>` <br/>
   `cargo run -- --ip <ip> --hostname <hostname> --ports <rang of ports(00-00)>` <br/>
