use std::sync::Arc;

use async_trait::async_trait;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres, Row};

use crate::services::json::{CityInformation, Properties};

#[async_trait]
pub trait PlaceRepository {
    async fn get_place_id(&self, country_code: &str, city_name: &str) -> Option<String>;

    async fn create_place_geometry(
        &self,
        place_id: &str,
        osm_id: usize,
        complex_polygon: &[u8],
        bounding_box: &[u8],
        center_point: &[u8],
    );

    async fn create_city_information(
        &self,
        place_id: &str,
        properties: Properties,
        country_code: &str,
        country_name: &str,
        city_name: &str,
    );
}

pub struct PostgresPlaceRepository {
    pool: Arc<Pool<Postgres>>,
}

impl PostgresPlaceRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlaceRepository for PostgresPlaceRepository {
    async fn get_place_id(&self, country_code: &str, city_name: &str) -> Option<String> {
        let sql = Query::select()
            .from(CityInformation::Table)
            .column(CityInformation::GooglePlaceId)
            .and_where(Expr::col(CityInformation::CountryCode).eq(country_code))
            .and_where(Expr::col(CityInformation::City).eq(city_name))
            .to_string(PostgresQueryBuilder);

        sqlx::query(sql.as_str())
            .fetch_optional(&*self.pool)
            .await
            .ok()
            .flatten()
            .map(|row| row.get(0))
    }

    async fn create_place_geometry(
        &self,
        place_id: &str,
        osm_id: usize,
        complex_polygon: &[u8],
        bounding_box: &[u8],
        center_point: &[u8],
    ) {
        let res = sqlx::query("INSERT INTO city_geom (google_place_id, osm_id, geom, bounding_box, center_point) VALUES ($1, $2, $3::geometry, $4::geometry, $5::geometry);")
            .bind(place_id)
            .bind(osm_id as i64)
            .bind(&complex_polygon)
            .bind(&bounding_box)
            .bind(&center_point)
            .execute(&*self.pool)
            .await;

        if res.is_err() {
            eprint!("create place geometry {:?}", res);
        }
    }

    async fn create_city_information(
        &self,
        place_id: &str,
        properties: Properties,
        country_code: &str,
        country_name: &str,
        city_name: &str,
    ) {
        let sql = Query::insert()
            .into_table(CityInformation::Table)
            .columns(vec![
                CityInformation::GooglePlaceId,
                CityInformation::OsmId,
                CityInformation::OsmType,
                CityInformation::PlaceId,
                CityInformation::PlaceRank,
                CityInformation::PlaceType,
                CityInformation::Importance,
                CityInformation::DisplayName,
                CityInformation::Category,
                CityInformation::CountryCode,
                CityInformation::Country,
                CityInformation::City,
            ])
            .values_panic(vec![
                place_id.into(),
                properties.osm_id.map(|x| x as i64).into(),
                properties.osm_type.into(),
                properties.place_id.map(|x| x as i64).into(),
                properties.place_rank.into(),
                properties.place_type.into(),
                properties.importance.into(),
                properties.display_name.into(),
                properties.category.into(),
                country_code.into(),
                country_name.into(),
                city_name.into(),
            ])
            .to_string(PostgresQueryBuilder);

        let res = sqlx::query(sql.as_str()).execute(&*self.pool).await;

        if res.is_err() {
            eprint!("create place geometry {:?}", res);
        }
    }
}
