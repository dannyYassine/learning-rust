use std::fmt::{self, Debug, Display, Formatter};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

trait PrettyPrint {
    fn pretty_display(self: &Self)
    where
        Self: Debug,
    {
        println!("This is pretty print display: {:?}", self);
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl PrettyPrint for Color {}

fn main() {
    let cities: [City; 3] = [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ];
    let colors: [Color; 3] = [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    for city in cities {
        println!("{}", city);
    }
    for color in colors {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
        color.pretty_display()
    }
}
