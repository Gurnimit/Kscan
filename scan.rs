use std::process::Command;

pub fn check_host(target: &str) -> bool {
    let output = Command::new("nmap")
        .arg("-sn")
        .arg(target)
        .output()
        .expect("Failed to execute nmap");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("Host is up")
}

pub fn parse_port_range(range: &str) -> Result<Vec<u16>, &'static str> {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        return Err("Invalid port range format");
    }
    let start: u16 = parts[0].parse().map_err(|_| "Invalid start port")?;
    let end: u16 = parts[1].parse().map_err(|_| "Invalid end port")?;
    if start > end {
        return Err("Start port must be less than or equal to end port");
    }
    Ok((start..=end).collect())
}

pub fn scan_port_with_nmap(ip: &str, port: u16) -> (String, String) {
    let output = Command::new("nmap")
        .arg("-sV")
        .arg("-p")
        .arg(&port.to_string())
        .arg(ip)
        .arg("-Pn")
        .output()
        .expect("Failed to execute nmap");
    let output_str = String::from_utf8_lossy(&output.stdout);
   for line in output_str.lines() {
       if line.contains("tcp") {
           let parts: Vec<&str> = line.split_whitespace().collect();
           if parts.len() >= 4 && parts[1] == "open" {
               let state = parts[1].to_string();
               let service = parts[2].to_string();
               let version = parts[3..].join(" ");
               return (state, format!("{} {}", service, version));
           }
       }
   }
   ("unknown".to_string(), "unknown".to_string())
}
