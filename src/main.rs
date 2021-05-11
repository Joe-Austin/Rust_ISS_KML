use std::{fmt, fs::File, io::Write};

use geo::Point;

mod geo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let pt = fetch_iss_location()?;
    let kml = create_kml(&pt);

    if args.len() > 1 {
        let mut file = File::create(&args[1])?;
        file.write_all(&kml.as_bytes())?;
    } else {
        println!("{}", kml)
    }
    
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

fn create_kml(point: &geo::Point) -> String {
    let kml = format!(
        "<kml xmlns=\"http://www.opengis.net/kml/2.2\">
    <Document>
    <name>ISS Location</name>
    <NetworkLink>
    <Link>
      <href>https://joeaustin.net/iss.kml</href>
      <refreshMode>onInterval</refreshMode>
      <refreshInterval>10</refreshInterval>
    </Link>
  </NetworkLink>
    <Style id=\"default\">
      <LabelStyle><scale>0</scale></LabelStyle>
      <IconStyle><Icon>
        <href>https://joeaustin.net/images/iss.png</href>
        <hotSpot x=\"0.5\" y=\"0.5\" xunits=\"fraction\" yunits=\"fraction\"/>
      </Icon></IconStyle>
    </Style>
    <Placemark>
      <name>ISS</name>
      <styleUrl>#default</styleUrl>
      <Point><coordinates>{},{},0</coordinates></Point>
    </Placemark>",
        point.lon, point.lat
    );

    return kml;
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
