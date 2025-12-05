use crate::Unit;
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Scale information for converting pixel measurements to real-world measurements
#[napi(object)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Scale {
  /// Pixel distance
  pub pixel_distance: f64,
  /// Real-world distance
  pub real_distance: f64,
  /// Unit for the real-world distance
  pub unit: Unit,
}

impl Scale {
  pub fn new(pixel_distance: f64, real_distance: f64, unit: Unit) -> Self {
    Self {
      pixel_distance,
      real_distance,
      unit,
    }
  }

  /// Calculate the scale ratio (pixels per unit)
  pub fn ratio(&self) -> f64 {
    if self.real_distance == 0.0 {
      0.0
    } else {
      self.pixel_distance / self.real_distance
    }
  }

  /// Convert a pixel distance to real-world distance
  pub fn pixel_to_real(&self, pixel_distance: f64) -> f64 {
    let ratio = self.ratio();
    if ratio == 0.0 {
      0.0
    } else {
      pixel_distance / ratio
    }
  }

  /// Convert a real-world distance to pixel distance
  pub fn real_to_pixel(&self, real_distance: f64) -> f64 {
    self.ratio() * real_distance
  }

  /// Convert a pixel distance to real-world distance in a specific unit
  pub fn pixel_to_real_unit(&self, pixel_distance: f64, target_unit: Unit) -> f64 {
    let real_in_scale_unit = self.pixel_to_real(pixel_distance);
    self.unit.convert(real_in_scale_unit, &target_unit)
  }
}
