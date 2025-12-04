use geo::{coord, Coord};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Represents a 2D point with floating point coordinates
#[napi(object)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

impl Point {
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }

  /// Calculate distance between two points
  pub fn distance_to(&self, other: &Point) -> f64 {
    let dx = self.x - other.x;
    let dy = self.y - other.y;
    (dx * dx + dy * dy).sqrt()
  }
}

impl From<Point> for Coord<f64> {
  fn from(p: Point) -> Self {
    // Coord::<f64>::new(p.x, p.y)
    coord! { x: p.x, y: p.y }
  }
}

impl From<Coord<f64>> for Point {
  fn from(c: Coord<f64>) -> Self {
    Point::new(c.x, c.y)
  }
}

/// Coordinate transformation for zoom and pan
#[napi(object)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
  pub scale: f64,
  pub offset_x: f64,
  pub offset_y: f64,
}

impl Transform {
  pub fn new(scale: f64, offset_x: f64, offset_y: f64) -> Self {
    Self {
      scale,
      offset_x,
      offset_y,
    }
  }

  pub fn identity() -> Self {
    Self {
      scale: 1.0,
      offset_x: 0.0,
      offset_y: 0.0,
    }
  }

  /// Transform a point from world coordinates to screen coordinates
  pub fn world_to_screen(&self, point: Point) -> Point {
    Point::new(
      point.x * self.scale + self.offset_x,
      point.y * self.scale + self.offset_y,
    )
  }

  /// Transform a point from screen coordinates to world coordinates
  pub fn screen_to_world(&self, point: Point) -> Point {
    Point::new(
      (point.x - self.offset_x) / self.scale,
      (point.y - self.offset_y) / self.scale,
    )
  }
}
