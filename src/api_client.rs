use std::fmt;
use std::convert::From;
use reqwest::blocking::get;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::Error as JsonError;

pub fn get_data(api_key: String, ip_address: String) -> Result<Response, MyError> {
    let request_url = format!("https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}", api_key, ip_address);
    let response = get(request_url)?;
    let code = response.status().as_u16();
    let mut body = String::new();
    if response.status().is_success() {
        body = response.text()?;
    }

    Ok(Response { code, body })
}

pub struct Response {
    pub code: u16,
    pub body: String,
}

impl Response {
    pub fn is_success(&self) -> bool {
        match self.code {
            200 => true,
            _=> false,
        }
    }

    pub fn parse_info(&self) -> Result<IPInfo, MyError> {
        let ip_info: IPInfo = serde_json::from_str(&self.body).map_err(MyError::from)?;

        Ok(ip_info)
    }
}

#[derive(Debug, Deserialize)]
pub struct IPInfo {
    pub city: String,
    pub country_name: String,
}

#[derive(Debug)]
pub enum MyError {
    Json(JsonError),
    Reqwest(ReqwestError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Json(err) => write!(f, "JSON Error: {}", err),
            MyError::Reqwest(err) => write!(f, "Reqwest Error: {}", err),
        }
    }
}

impl From<JsonError> for MyError {
    fn from(error: JsonError) -> Self {
        MyError::Json(error)
    }
}

impl From<ReqwestError> for MyError {
    fn from(error: ReqwestError) -> Self {
        MyError::Reqwest(error)
    }
}
