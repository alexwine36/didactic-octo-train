use napi_derive::napi;
use serde::{Deserialize, Serialize};
use uom::si::area::{square_centimeter, square_foot, square_inch, square_meter, square_yard};
use uom::si::f64::{Area, Length};
use uom::si::length::{centimeter, foot, inch, meter, yard};
/// Measurement units supported by the system
#[napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Unit {
  /// Imperial units
  Yards,
  Feet,
  Inches,
  /// Metric units
  Meters,
  Centimeters,
}

impl Unit {
  pub fn get_unit(&self, value: f64) -> Length {
    match self {
      Unit::Yards => Length::new::<yard>(value),
      Unit::Feet => Length::new::<foot>(value),
      Unit::Inches => Length::new::<inch>(value),
      Unit::Meters => Length::new::<meter>(value),
      Unit::Centimeters => Length::new::<centimeter>(value),
    }
  }

  pub fn get_area_unit(&self, value: f64) -> Area {
    match self {
      Unit::Yards => Area::new::<square_yard>(value),
      Unit::Feet => Area::new::<square_foot>(value),
      Unit::Inches => Area::new::<square_inch>(value),
      Unit::Meters => Area::new::<square_meter>(value),
      Unit::Centimeters => Area::new::<square_centimeter>(value),
    }
  }

  /// Convert a value from one unit to another
  pub fn convert(&self, value: f64, to: &Unit) -> f64 {
    let from = self.get_unit(value);

    match to {
      Unit::Yards => from.get::<yard>(),
      Unit::Feet => from.get::<foot>(),
      Unit::Inches => from.get::<inch>(),
      Unit::Meters => from.get::<meter>(),
      Unit::Centimeters => from.get::<centimeter>(),
    }
  }

  pub fn convert_area(&self, value: f64, to: &Unit) -> f64 {
    let from = self.get_area_unit(value);

    match to {
      Unit::Yards => from.get::<square_yard>(),
      Unit::Feet => from.get::<square_foot>(),
      Unit::Inches => from.get::<square_inch>(),
      Unit::Meters => from.get::<square_meter>(),
      Unit::Centimeters => from.get::<square_centimeter>(),
    }
  }

  /// Get the display string for this unit
  pub fn display(&self) -> &'static str {
    match self {
      Unit::Yards => "yd",
      Unit::Feet => "ft",
      Unit::Inches => "in",
      Unit::Meters => "m",
      Unit::Centimeters => "cm",
    }
  }

  pub fn unit_str(&self) -> &'static str {
    match self {
      Unit::Yards => "Yards",
      Unit::Feet => "Feet",
      Unit::Inches => "Inches",
      Unit::Meters => "Meters",
      Unit::Centimeters => "Centimeters",
    }
  }
}

/// Unit conversion utilities
pub struct UnitUtils;

impl UnitUtils {
  /// Convert a value from one unit to another
  pub fn convert(value: f64, from: Unit, to: Unit) -> f64 {
    from.convert(value, &to)
  }
  pub fn convert_area(value: f64, from: Unit, to: Unit) -> f64 {
    from.convert_area(value, &to)
  }

  /// Get all available units
  pub fn all_units() -> Vec<Unit> {
    vec![
      Unit::Yards,
      Unit::Feet,
      Unit::Inches,
      Unit::Meters,
      Unit::Centimeters,
    ]
  }

  /// Get imperial units
  pub fn imperial_units() -> Vec<Unit> {
    vec![Unit::Yards, Unit::Feet, Unit::Inches]
  }

  /// Get metric units
  pub fn metric_units() -> Vec<Unit> {
    vec![Unit::Meters, Unit::Centimeters]
  }
}
