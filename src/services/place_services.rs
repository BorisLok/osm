use std::borrow::Borrow;
use std::str::FromStr;

use futures::TryFutureExt;
use geo::GeometryCollection;
use geojson::{Feature, GeoJson, quick_collection};
use reqwest::header::HeaderMap;

use crate::services::json::{AutoCompleteResponse, GeometryResponse, Properties};

pub struct GoogleServices;

impl GoogleServices {
    pub async fn get_place_id(
        country_name: &str,
        city_name: &str,
        latitude: f64,
        longitude: f64,
        api_key: &str,
    ) -> Option<String> {
        let types = "geocode";
        let input = format!("{}, {}", city_name, country_name);
        let location = format!("{} {}", latitude, longitude);

        let url = format!("https://maps.googleapis.com/maps/api/place/autocomplete/json?input={}&key={}&location={}&types={}", input, api_key, location, types);

        reqwest::Client::new()
            .get(&url)
            .send()
            .and_then(|res| async move { res.text().await })
            .await
            .map(|response| {
                serde_json::from_str::<AutoCompleteResponse>(&response)
                    .map(|auto_complete| {
                        if auto_complete.predictions.is_empty() {
                            return None;
                        }
                        auto_complete
                            .predictions
                            .first()
                            .map(|x| x.place_id.to_owned())
                    })
                    .ok()
                    .flatten()
            })
            .ok()
            .flatten()
    }

    pub async fn get_detail_by_place_id(place_id: &str, api_key: &str) -> Option<GeometryResponse> {
        let fields = vec!["formatted_address", "geometry", "name"].join(",");
        let url = format!(
            "https://maps.googleapis.com/maps/api/place/details/json?placeid={}&fields={}&key={}",
            place_id, fields, api_key
        );

        reqwest::Client::new()
            .get(&url)
            .send()
            .and_then(|res| async move { res.text().await })
            .await
            .map(|response| serde_json::from_str::<GeometryResponse>(&response).ok())
            .ok()
            .flatten()
    }
}

pub struct OSMServices;

impl OSMServices {
    pub async fn get_detail_by_osm(
        country_name: &str,
        city_name: &str,
    ) -> Option<Vec<(Properties, Vec<u8>)>> {
        let input = format!("{}, {}", city_name, country_name);
        let url = format!(
            "https://nominatim.openstreetmap.org/search.php?q={}&polygon_geojson=1&format=geojson",
            &input
        );

        reqwest::Client::new()
            .get(&url)
            .headers(constant_headers())
            .send()
            .and_then(|response| async move {
                response
                    .text()
                    .await
                    .map(|text| GeoJson::from_str(&text).ok())
            })
            .await
            .ok()
            .flatten()
            .map(|geo_json| parse_geojson(&geo_json))
            .flatten()
    }
}

fn constant_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(reqwest::header::ACCEPT, "*/*".try_into().unwrap());
    headers.append(
        reqwest::header::ACCEPT_LANGUAGE,
        "en-US".try_into().unwrap(),
    );
    headers.append(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0".try_into().unwrap(),
    );
    headers
}

fn parse_geojson(json: &GeoJson) -> Option<Vec<(Properties, Vec<u8>)>> {
    match json {
        GeoJson::Geometry(_) => None,
        GeoJson::Feature(feature) => {
            let res = parse_geo_feature(feature);
            if let (Some(properties), Some(geom)) = res {
                return Some(vec![(properties, geom)]);
            }
            None
        }
        GeoJson::FeatureCollection(features) => {
            let mut v: Vec<(Properties, Vec<u8>)> = Vec::new();
            for feature in &features.features {
                let res = parse_geo_feature(feature);
                if let (Some(properties), Some(geom)) = res {
                    v.push((properties, geom));
                }
            }

            if v.is_empty() {
                None
            } else {
                Some(v)
            }
        }
    }
}

fn parse_geo_feature(feature: &Feature) -> (Option<Properties>, Option<Vec<u8>>) {
    let geojson = GeoJson::Feature(feature.clone());
    let properties = feature.properties.borrow();
    if let Some(properties) = properties {
        let properties: Option<Properties> = serde_json::to_string(properties)
            .ok()
            .map(|x| serde_json::from_str(x.as_str()).ok())
            .flatten();

        if let Some(properties) = properties {
            if properties.category == Some("boundary".to_string()) {
                let geometry: GeometryCollection<f64> =
                    quick_collection(&geojson).expect("Can parse geometry collection.");
                let gc = geo::Geometry::GeometryCollection(geometry);
                let bytes = wkb::geom_to_wkb(&gc).expect("Can encode geom to bytes.");

                return (Some(properties), Some(bytes));
            }
        }
    }

    (None, None)
}
