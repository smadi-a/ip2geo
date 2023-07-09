use dirs;
use std::fs;
use std::env;
use regex::Regex;
use reqwest::Error;
use std::path::Path;
use reqwest::blocking::get;

const VARS_FILE: &str = ".ip2geo";
struct Response {
    code: u16,
    body: String,
}

impl Response {
    fn is_success(self) -> bool {
        match self.code {
            200 => true,
            _=> false,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit_gracefully("Which IP address do you need to lookup?\nFor example, try 'ip2geo 210.50.134.58'\n\n\nip2geo uses ipgeolocation.io\nDon't forget to add your API key in your ip2geo dot file");
    }

    let ip_address: &str = &args[1];
    if !is_valid_ip_address(ip_address) {
        exit_gracefully("Invalid IP address provided");
    }

    let api_key: String = get_api_key();
    if api_key.is_empty() {
        exit_gracefully("No API key provided");
    }

    match get_data(api_key, ip_address.to_string()) {
        Ok(response) => {
            if response.is_success() {
                println!("We're doing it!");
            } else {
                println!("Request failed");
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

fn get_data(api_key: String, ip_address: String) -> Result<Response, Error> {
    let request_url = format!("https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}", api_key, ip_address);
    let response = get(request_url)?;
    let code = response.status().as_u16();
    let mut body = String::new();
    if response.status().is_success() {
        body = response.text()?;
    }

    Ok(Response { code, body })
}
