// no need for std:: before Formatter or Display
use std::fmt::{self, Formatter, Display};

struct City {
    // string with static lifetime aka for the lifetime of the program
    name: &'static str,
    // Lat
    lat: f32,
    // Long
    lon: f32,
}

impl Display for City {
    // f is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' }; // inline if, similar to ternary operator
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // 3 decimal points
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({:>3}, {:>3}, {:>3}) ", self.red, self.green, self.blue);
        write!(f, "0x{:>02X}{:>02X}{:>02X}", self.red, self.green, self.blue)
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
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() { // iterates over &T aka the addresses of the items in the above list
        // do the below, then call next() and do it again
        // Switch this to use {} once you've added an implementation
        // for fmt::Display
        println!("{}", *color)
    }
}
