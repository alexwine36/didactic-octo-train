use crate::takeoff_core::Point;
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Represents a line segment between two points
#[napi(object)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Line {
  pub start: Point,
  pub end: Point,
}

impl Line {
  pub fn new(start: Point, end: Point) -> Self {
    Self { start, end }
  }

  /// Calculate the length of the line
  pub fn length(&self) -> f64 {
    self.start.distance_to(&self.end)
  }
}

#[napi(object)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polyline {
  pub points: Vec<Point>,
}

impl Polyline {
  pub fn new(points: Vec<Point>) -> Self {
    Self { points }
  }
  pub fn length(&self) -> f64 {
    let mut length = 0.0;
    for i in 0..self.points.len() - 1 {
      length += self.points[i].distance_to(&self.points[i + 1]);
    }
    length
  }
}

impl From<Vec<Point>> for Polyline {
  fn from(points: Vec<Point>) -> Self {
    Self { points }
  }
}

impl From<Polyline> for Vec<Point> {
  fn from(polyline: Polyline) -> Self {
    polyline.points
  }
}
