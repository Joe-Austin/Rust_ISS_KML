use std::fmt;

use geo::Point;

mod geo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pt = fetch_iss_location()?;
    println!("{}", pt);
    Ok(())
}

fn fetch_iss_location() -> Result<geo::Point, FetchError> {
    match reqwest::blocking::get("http://api.open-notify.org/iss-now.json") {
        Ok(response) => match response.text() {
            Ok(text) => return parse_point_from_json(&text),
            Err(_) => return Err(FetchError::InvalidResponse),
        },
        Err(_) => return Err(FetchError::RequestFailedError),
    }
}

fn parse_point_from_json(json: &String) -> Result<geo::Point, FetchError> {
    //let json = json::parse(&resp).unwrap();
    match json::parse(&json) {
        Ok(value) => {
            let is_success = value["message"] == "success";
            let lat_string = value["iss_position"]["latitude"].as_str();
            let lon_string = value["iss_position"]["longitude"].as_str();
            if is_success && !lat_string.is_none() && !lon_string.is_none() {
                let lat = lat_string.unwrap().parse::<f64>().unwrap();
                let lon = lon_string.unwrap().parse::<f64>().unwrap();

            Ok(Point::new(lat, lon))
            } else {
                return Err(FetchError::ParserError);
            }
        }
        Err(_) => return Err(FetchError::ParserError),
    }
}

#[derive(Debug)]
enum FetchError {
    RequestFailedError,
    InvalidResponse,
    ParserError,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FetchError::RequestFailedError => write!(f, "request was unsuccessful"),
            FetchError::InvalidResponse => write!(f, "the received response was not successful"),
            FetchError::ParserError => write!(f, "unable to parse the response"),
        }
    }
}

impl std::error::Error for FetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FetchError::RequestFailedError => None,
            FetchError::InvalidResponse => None,
            FetchError::ParserError => None,
        }
    }
}
