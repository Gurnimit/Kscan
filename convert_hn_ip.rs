use std::net::{ToSocketAddrs, IpAddr};

pub fn resolve_hostname(hostname: &str) -> Result<Vec<IpAddr>, String> {
    match (hostname, 0).to_socket_addrs() {
        Ok(addrs) => {
            let ips: Vec<IpAddr> = addrs.map(|addr| addr.ip()).collect();
            if ips.is_empty() {
                Err(format!("No IP addresses found for hostname: {}", hostname))
            } else {
                Ok(ips)
            }
        },
        Err(_) => Err(format!("Could not resolve hostname: {}", hostname)),
    }
}

