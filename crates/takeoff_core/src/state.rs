use crate::{Measurement, Scale, Transform};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// State management for takeoff measurements
#[napi(object)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeoffState {
  pub measurements: Vec<Measurement>,
  pub scale: Option<Scale>,
  pub transform: Transform,
  pub count: u32,
  /// File ID for PDF files (null for images)
  pub file_id: Option<String>,
  /// Page number for PDF files (null for images)
  pub page_number: Option<u32>,
}

// #[napi(object)]
impl TakeoffState {
  pub fn new() -> Self {
    Self {
      measurements: Vec::new(),
      scale: None,
      transform: Transform::identity(),
      count: 0,
      file_id: None,
      page_number: None,
    }
  }

  pub fn add_measurement(&mut self, measurement: Measurement) {
    let is_count = matches!(measurement, Measurement::Count { .. });
    self.measurements.push(measurement);
    if is_count {
      self.count += 1;
    }
  }

  pub fn remove_measurement(&mut self, id: &str) -> bool {
    let initial_len = self.measurements.len();
    self.measurements.retain(|m| m.id() != id);
    let removed = self.measurements.len() < initial_len;
    if removed {
      // Recalculate count
      self.count = self
        .measurements
        .iter()
        .filter(|m| matches!(m, Measurement::Count { .. }))
        .count() as u32;
    }
    removed
  }

  pub fn clear_measurements(&mut self) {
    self.measurements.clear();
    self.count = 0;
  }

  pub fn set_scale(&mut self, scale: Scale) {
    self.scale = Some(scale);
  }

  pub fn update_transform(&mut self, transform: Transform) {
    self.transform = transform;
  }
}

impl Default for TakeoffState {
  fn default() -> Self {
    Self::new()
  }
}
