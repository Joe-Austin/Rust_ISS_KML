use std::fmt;

#[derive(Debug)]
pub struct Point{
    pub lat : f64,
    pub lon : f64,
}

impl Point {
    pub fn new(lat : f64, lon : f64)  -> Point {
        Point{lat, lon}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[latitude: {}, longitude: {}]", self.lat, self.lon)
    }
}