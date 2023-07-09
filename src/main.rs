use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit_gracefully("Which IP address do you need to lookup?\nFor example, try 'ip2geo 210.50.134.58'\n\n\nip2geo uses ipgeolocation.io\nDon't forget to add your API key in your ip2geo dot file");
    }

    let ip_address: &str = &args[1];
    if !is_valid_ip_address(ip_address) {
        exit_gracefully("Invalid IP address provided");
    }
}

fn exit_gracefully(message: &str) {
    println!("{}", message);
    std::process::exit(0);
}

fn is_valid_ip_address(ip: &str) -> bool {
    let ip_regex = Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap();
    ip_regex.is_match(ip)
}
