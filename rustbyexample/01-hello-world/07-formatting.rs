use std::fmt::{Display, Formatter, Result};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lat_dir = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_dir = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{}, {:.3}°{}", self.name,
            self.lat.abs(), lat_dir,
            self.lon.abs(), lon_dir
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    blue: u8,
    green: u8,
}

impl Color {
    fn get_hex_value(val: u8) -> String {
        format!("{:X}", val)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "0x");

        write!(f, "{}", format!("{:X}", self.red));
        write!(f, "{}", format!("{:X}", self.blue));
        write!(f, "{}", format!("{:X}", self.green))
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    println!("");

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
        println!("{}", *color);
    }
}