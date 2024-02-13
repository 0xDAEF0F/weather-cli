use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub results: Option<Vec<Location>>,
    pub generationtime_ms: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    id: i64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    elevation: f64,
    feature_code: String,
    country_code: String,
    admin1_id: Option<i64>,
    admin2_id: Option<i64>,
    timezone: String,
    population: Option<i64>,
    country_id: i64,
    pub country: String,
    pub admin1: Option<String>,
    admin2: Option<String>,
}

impl Location {
    pub fn get_coordinates(&self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }
}
