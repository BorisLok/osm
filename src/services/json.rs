use geo::point;
use sea_query::Iden;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct AutoCompleteResponse {
    pub(crate) predictions: Vec<Place>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Place {
    pub(crate) place_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GeometryResponse {
    pub result: Result,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    pub(crate) geometry: Geometry,
}

impl Result {
    pub fn center(&self) -> Vec<u8> {
        let point = point!(x: self.geometry.location.lng, y: self.geometry.location.lat);
        wkb::geom_to_wkb(&geo::Geometry::from(point)).expect("Can parse center point to wkb")
    }

    pub fn bounding_box(&self) -> Vec<u8> {
        let northeast_point = point!(x: self.geometry.viewport.northeast.lng, y: self.geometry.viewport.northeast.lat);
        let southwest_point = point!(x: self.geometry.viewport.southwest.lng, y: self.geometry.viewport.southwest.lat);
        let bbox = geo::Rect::new(northeast_point, southwest_point);
        let polygon = geo::Polygon::from(bbox);
        wkb::geom_to_wkb(&geo::Geometry::from(polygon)).expect("Can parse viewport to wkb.")
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Geometry {
    pub(crate) location: _Coordinate,
    pub(crate) viewport: ViewPort,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ViewPort {
    pub(crate) northeast: _Coordinate,
    pub(crate) southwest: _Coordinate,
}

#[derive(Debug, Deserialize)]
pub(crate) struct _Coordinate {
    pub(crate) lat: f64,
    pub(crate) lng: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Properties {
    pub osm_id: Option<usize>,
    pub osm_type: Option<String>,
    pub place_id: Option<usize>,
    pub place_rank: Option<i16>,
    #[serde(rename(deserialize = "type"))]
    pub place_type: Option<String>,
    pub importance: Option<f64>,
    pub display_name: Option<String>,
    pub category: Option<String>,
}

#[derive(Iden)]
pub(crate) enum CityInformation {
    Table,
    GooglePlaceId,
    OsmId,
    OsmType,
    PlaceId,
    PlaceRank,
    PlaceType,
    Importance,
    DisplayName,
    Category,
    CountryCode,
    Country,
    City,
}
