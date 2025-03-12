#![allow(unused)]

// This Rust program defines a `Coordinates` struct to represent geographical coordinates (latitude and longitude) and implements methods for calculating the distance between two points on the Earth's surface using the Haversine formula. 

struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl Coordinates {
    fn new(latitude: f64, longitude: f64) -> Self {
        Coordinates { latitude, longitude }
    }
    fn distance_to(&self, other: &Self) -> f64 {
        let earth_radius = 6371.0; // in kilometers
        let dlat = (other.latitude - self.latitude).to_radians(); 
        let dlon = (other.longitude - self.longitude).to_radians();
        let a = (dlat / 2.0).sin().powi(2) + self.latitude.to_radians().cos() * other.latitude.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt()); 
        earth_radius * c
    }
}
fn main() {
    let coord1 = Coordinates::new(52.5200, 13.4050); // Berlin
    let coord2 = Coordinates::new(48.8566, 2.3522); // Paris
    let coord3 = Coordinates::new(35.6895, 139.6917); // Tokyo
    let coord4 = Coordinates::new(47.4979, 12.3964); // Rome
    let coord6 = Coordinates::new(14.0, 120.5); // Manila
    let coord7 = Coordinates::new(22.56, 113.89); // Shenzhen

    let coord8 = {
        Coordinates { // Zurich
            latitude : 47.0,
            longitude: 8.5,
        }
    }; 
    

    println!("Distance between Zurich and Paris: {} km", format!("{:.4}", coord8.distance_to(&coord2))); 
}   


