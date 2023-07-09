mod api_client;
use std::fs;
use std::env;
use std::path::Path;
use regex::Regex;

const VARS_FILE: &str = ".ip2geo";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit_gracefully("Which IP address do you need to lookup?\nFor example, try 'ip2geo 87.172.253.31'\n\n\nip2geo uses ipgeolocation.io\nDon't forget to add your API key in your ip2geo dot file");
    }

    let ip_address: &str = &args[1];
    if !is_valid_ip_address(ip_address) {
        exit_gracefully("Invalid IP address provided");
    }

    let api_key: String = get_api_key();
    if api_key.is_empty() {
        exit_gracefully("No API key provided");
    }

    match api_client::get_data(api_key, ip_address.to_string()) {
        Ok(response) => {
            if response.is_success() {
                match response.parse_info() {
                    Ok(ip_info) => {
                        println!("{} - {}", ip_info.city, ip_info.country_name);
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                println!("Request failed with code {}", response.code);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
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

fn get_api_key() -> String {
    let home_dir = dirs::home_dir().expect("Failed to get the home directory");
    let vars_file_path = home_dir.join(VARS_FILE);
    if !Path::new(&vars_file_path).exists() {
        return String::new();
    }

    let api_key: String = fs::read_to_string(vars_file_path).expect("Failed to read API key");

    api_key
}
