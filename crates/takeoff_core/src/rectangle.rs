use crate::Point;
use geo::{Area, Coord, Polygon as GeoPolygon, Rect};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Represents a rectangle defined by two points
#[napi(object)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
  pub start: Point,
  pub end: Point,
}

impl Rectangle {
  pub fn new(start: Point, end: Point) -> Self {
    Self { start, end }
  }

  /// Calculate the width of the rectangle
  pub fn width(&self) -> f64 {
    (self.end.x - self.start.x).abs()
  }

  /// Calculate the height of the rectangle
  pub fn height(&self) -> f64 {
    (self.end.y - self.start.y).abs()
  }

  /// Calculate the area of the rectangle
  pub fn area(&self) -> f64 {
    self.to_polygon().unsigned_area()
  }

  /// Calculate the perimeter of the rectangle
  pub fn perimeter(&self) -> f64 {
    2.0 * (self.width() + self.height())
  }

  pub fn to_polygon(&self) -> GeoPolygon<f64> {
    let start: Coord<f64> = self.start.into();
    let end: Coord<f64> = self.end.into();
    let rect = Rect::new(start, end);
    rect.to_polygon()
  }
}
