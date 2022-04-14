use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Countries {
    pub countries: Vec<Country>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub country: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub cities: Vec<City>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub region: String,
    pub name: String,
}