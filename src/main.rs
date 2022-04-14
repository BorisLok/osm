#![warn(unused_variables)]

use std::sync::Arc;

use crate::core::environment::Environment;
use crate::core::json::Countries;
use crate::core::postgres_connection::PostgresConnection;
use crate::services::place_services::{GoogleServices, OSMServices};
use crate::services::repo::{PlaceRepository, PostgresPlaceRepository};

mod core;
mod services;

#[tokio::main]
async fn main() {
    let env = Environment::new();
    let database = PostgresConnection::new(&env)
        .await
        .expect("Can connect to database.");
    let repository = PostgresPlaceRepository::new(Arc::clone(&database.pool));

    let file = tokio::fs::read("./cities.yaml")
        .await
        .expect("Can read cities yaml");

    let countries = serde_yaml::from_slice::<Countries>(&file).expect("Decode yaml failed");

    for country in &countries.countries {
        let country_code = &*country.country_code;
        let country_name = &*country.country;
        let latitude = country.latitude;
        let longitude = country.longitude;

        for city in &country.cities {
            let place_id = repository.get_place_id(country_code, &city.region).await;
            if place_id.is_some() {
                continue;
            }

            let place_id = match place_id {
                None => {
                    GoogleServices::get_place_id(
                        country_name,
                        &city.region,
                        latitude,
                        longitude,
                        &env.api_key,
                    )
                    .await
                }
                Some(place_id) => Some(place_id),
            };

            if let Some(place_id) = place_id {
                let place_detail =
                    GoogleServices::get_detail_by_place_id(&place_id, &env.api_key).await;
                let osm_information =
                    OSMServices::get_detail_by_osm(country_name, &city.region).await;

                if let (Some(geometries), Some(place_detail)) = (osm_information, place_detail) {
                    let bounding_box = place_detail.result.bounding_box();
                    let center_point = place_detail.result.center();

                    for geometry in geometries {
                        let osm_id = geometry.0.osm_id.unwrap();
                        repository
                            .create_place_geometry(
                                &place_id,
                                osm_id,
                                &geometry.1,
                                &bounding_box,
                                &center_point,
                            )
                            .await;

                        repository
                            .create_city_information(
                                &place_id,
                                geometry.0,
                                country_code,
                                country_name,
                                &city.region,
                            )
                            .await;
                    }
                }
            } else {
                eprint!(
                    "country {:?}, city: {:?} can't get place id.",
                    country_name, city
                );
            }
        }
    }
}
