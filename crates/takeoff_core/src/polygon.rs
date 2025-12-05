use crate::Point;
use geo::{Area, Coord, LineString, Polygon as GeoPolygon};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Represents a polygon defined by a list of points
#[napi(object)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
  pub points: Vec<Point>,
}

impl Polygon {
  pub fn new(points: Vec<Point>) -> Self {
    Self { points }
  }

  fn to_polygon(&self) -> GeoPolygon<f64> {
    let points: Vec<Coord<f64>> = self.points.iter().map(|p| (*p).into()).collect();
    GeoPolygon::new(LineString::from(points), vec![])
  }

  pub fn area(&self) -> f64 {
    self.to_polygon().unsigned_area()
  }

  pub fn perimeter(&self) -> f64 {
    let mut perimeter = 0.0;
    for i in 0..self.points.len() {
      let j = (i + 1) % self.points.len();
      perimeter += self.points[i].distance_to(&self.points[j]);
    }
    perimeter
  }
}
