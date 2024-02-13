use serde::{Deserialize, Serialize};

// first part: name to locations
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

// second part: location to weather data
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseTwo {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    hourly_units: HourlyUnits,
    hourly: HourlyData,
}

#[derive(Serialize, Deserialize, Debug)]
struct HourlyUnits {
    time: String,
    temperature_2m: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HourlyData {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}

impl ApiResponseTwo {
    pub fn get_current_temperature(&self) -> Option<&f64> {
        self.hourly.temperature_2m.first()
    }
}
