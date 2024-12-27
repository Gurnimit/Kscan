use clap::{Arg, Command};
use std::sync::{Arc, Mutex};
use std::thread;
mod scan;
mod convert_hn_ip;

fn main() {
println!("======================================");
    let matches = Command::new("Port Scanner")
        .version("1.0")
        .author("Your Name")
        .about("Scans a range of TCP ports on a given IP address or hostname and identifies open services with their versions")
        .arg(
            Arg::new("IP")
                .long("ip")
                .value_name("IP")
                .help("Sets the IP address to scan")
        )
        .arg(
            Arg::new("HOSTNAME")
                .long("hostname")
                .value_name("HOSTNAME")
                .help("Sets the hostname to scan")
        )

        .arg(
            Arg::new("PORT_RANGE")
                .short('p')
                .long("ports")
                .value_name("PORT_RANGE")
                .help("Sets the range of ports to scan (e.g., 20-80)")
                .required(true)
        )
        .get_matches();
 let ip_opt = matches.get_one::<String>("IP").map(|s| s.to_string());
    let hostname_opt = matches.get_one::<String>("HOSTNAME").map(|s| s.to_string());
    let port_range = matches.get_one::<String>("PORT_RANGE").expect("Port range is required").to_string();

    if ip_opt.is_none() && hostname_opt.is_none() {
        println!("You must provide either an IP address, a hostname, or both.");
        return;
    }

    if let Some(ip) = ip_opt {
        scan_and_print_results(ip, port_range.clone());
    }

    if let Some(hostname) = hostname_opt {
        match convert_hn_ip::resolve_hostname(&hostname) {
            Ok(ip_addrs) => {
                for ip in ip_addrs.iter() {
                    scan_and_print_results(ip.to_string(), port_range.clone());
                }
            },
            Err(e) => println!("{}", e),
        }
    }
}

fn scan_and_print_results(target: String, port_range: String) {
    if scan::check_host(&target) {
        println!("Host {} is up", target);
println!("======================================");

        let ports: Vec<u16> = scan::parse_port_range(&port_range).expect("Invalid port range");

        let target_arc = Arc::new(target.to_string());
        let results = Arc::new(Mutex::new(Vec::new()));

        let mut handles = vec![];
        let chunk_size = 10; // Define a reasonable chunk size for each thread

        for chunk in ports.chunks(chunk_size) {
            let target = Arc::clone(&target_arc);
            let results = Arc::clone(&results);
            let chunk = chunk.to_vec();
            let handle = thread::spawn(move || {
                for port in chunk {
                    let tcp_state = scan::scan_port_with_nmap(&target, port);
                    if tcp_state.0 == "open" {
                        let mut results = results.lock().unwrap();
                        results.push((port, tcp_state));
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }


        let results = results.lock().unwrap();
        print_results(&results);
    } else {
        println!("Host {} is down or unreachable.", target);
    }
}

fn print_results(results: &[(u16, (String, String))]) {
    println!("\n+--------+----------+------------------------------------+");
    println!("|  PORT  |  STATE   | SERVICE                            |");
    println!("+--------+----------+------------------------------------+");
    for (port, (state, service)) in results.iter() {
        println!("| {:<6} | {:<8} | {:<34} |", format!("{}/tcp", port), state, service);
    }
println!("+--------+----------+------------------------------------+");
}
