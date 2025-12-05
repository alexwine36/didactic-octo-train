use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use takeoff_core::{Measurement, Scale, Unit};

#[napi]
#[derive(Serialize, Deserialize, Clone)]
pub struct TakeoffMeasurement {
  measurement: Measurement,
}

#[napi]
impl TakeoffMeasurement {
  #[napi(constructor)]
  pub fn new(measurement: Measurement) -> Self {
    Self { measurement }
  }

  #[napi(getter)]
  pub fn measurement(&self) -> Measurement {
    self.measurement.clone()
  }
  #[napi(setter)]
  pub fn set_measurement(&mut self, measurement: Measurement) {
    self.measurement = measurement;
  }

  #[napi]
  pub fn calculate_area(&self, scale: Scale, target_unit: Unit) -> Result<f64> {
    let measurement = self.measurement.clone();
    let real_area = measurement
      .real_area_with_target(&scale.clone().into(), &target_unit.into())
      .ok_or_else(|| Error::from_reason("Failed to calculate area"))?;

    Ok(real_area)
  }

  #[napi]
  pub fn calculate_perimeter(&self, scale: Scale, target_unit: Unit) -> Result<f64> {
    let measurement = self.measurement.clone();
    let pixel_perimeter = measurement
      .pixel_perimeter()
      .ok_or_else(|| Error::from_reason("Measurement is not an area measurement"))?;

    let real_perimeter = scale.pixel_to_real_unit(pixel_perimeter, target_unit);

    Ok(real_perimeter)
  }

  #[napi]
  pub fn calculate_distance(&self, scale: Scale, target_unit: Unit) -> Result<f64> {
    let measurement = self.measurement.clone();
    let pixel_distance = measurement
      .pixel_length()
      .ok_or_else(|| Error::from_reason("Measurement is not a linear measurement"))?;

    let real_distance = scale.pixel_to_real_unit(pixel_distance, target_unit);

    Ok(real_distance)
  }
}
