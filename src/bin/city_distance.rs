

struct Cities {
    name: String,
    coordinates: Coordinates,
}

struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl Coordinates {
    fn new(latitude: f64, longitude: f64) -> Self {
        Coordinates { latitude, longitude }
    }

    fn distance_to(&self, other: &Self) -> f64 {
        haversine_distance(self.latitude, self.longitude, other.latitude, other.longitude)
    }
}

impl Cities {
    fn new(name: &str, latitude: f64, longitude: f64) -> Self {
        Cities {
            name: name.to_string(),
            coordinates: Coordinates::new(latitude, longitude),
        }
    }

    fn distance_to(&self, other: &Self) -> f64 {
        self.coordinates.distance_to(&other.coordinates)
    }
}


fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let earth_radius = 6371.0; // in kilometers
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    earth_radius * c
}

fn main() {
    let berlin = Cities::new("Berlin", 52.5200, 13.4050);
    let paris = Cities::new("Paris", 48.8566, 2.3522);
    let tokyo = Cities::new("Tokyo", 35.6895, 139.6917);
    let rome = Cities::new("Rome", 47.4979, 12.3964);

    println!(
        "Distance between {} and {}: {} km",
        tokyo.name,
        paris.name,
        tokyo.distance_to(&paris)
    );
    println!(
        "Distance between {} and {}: {} km",
        rome.name,
        tokyo.name,
        rome.distance_to(&tokyo)
    );
    println!(
        "Distance between {} and {}: {} km",
        berlin.name,
        rome.name,
        berlin.distance_to(&rome)
    );

    // Example using Coordinates directly
    let coord1 = Coordinates::new(52.5200, 13.4050); // Berlin
    let coord2 = Coordinates::new(48.8566, 2.3522); // Paris
    println!("Distance between Berlin and Paris: {} km", coord1.distance_to(&coord2));