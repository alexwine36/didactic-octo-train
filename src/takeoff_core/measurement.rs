use crate::takeoff_core::{Line, Point, Polygon, Polyline, Rectangle, Scale, Unit, UnitUtils};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

/// Types of measurements that can be created
#[napi(discriminant = "type")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Measurement {
  /// Linear measurement (line or polyline)
  Linear {
    id: String,
    line: Option<Line>,         // Single line segment (for backward compatibility)
    polyline: Option<Polyline>, // Multiple connected line segments
    scale: Scale,
    display_unit: Unit,
  },
  /// Area measurement (rectangle or polygon)
  Area {
    id: String,
    rectangle: Option<Rectangle>,
    polygon: Option<Polygon>,
    scale: Scale,
    display_unit: Unit,
  },
  /// Count marker
  Count { id: String, point: Point },
}

impl Measurement {
  /// Get the ID of the measurement
  pub fn id(&self) -> &str {
    match self {
      Measurement::Linear { id, .. } => id,
      Measurement::Area { id, .. } => id,
      Measurement::Count { id, .. } => id,
    }
  }

  /// Calculate pixel area for area measurements
  pub fn pixel_area(&self) -> Option<f64> {
    match self {
      Measurement::Area {
        rectangle, polygon, ..
      } => {
        if let Some(rect) = rectangle {
          Some(rect.area())
        } else if let Some(poly) = polygon {
          Some(poly.area())
        } else {
          None
        }
      }
      _ => None,
    }
  }

  pub fn real_area_with_target(&self, scale: &Scale, target_unit: &Unit) -> Option<f64> {
    match self {
      Measurement::Area { .. } => {
        let pixel_area = self.pixel_area().unwrap_or(0.0);
        let scale_ratio = scale.ratio();
        let real_area_sq = if scale_ratio == 0.0 {
          0.0
        } else {
          pixel_area / (scale_ratio * scale_ratio)
        };
        let real_area = match target_unit {
          Unit::Yards => UnitUtils::convert_area(real_area_sq, scale.unit, Unit::Yards),
          Unit::Feet => UnitUtils::convert_area(real_area_sq, scale.unit, Unit::Feet), //real_area_sq_meters / (0.3048 * 0.3048),
          Unit::Inches => UnitUtils::convert_area(real_area_sq, scale.unit, Unit::Inches), //real_area_sq_meters / (0.0254 * 0.0254),
          Unit::Meters => UnitUtils::convert_area(real_area_sq, scale.unit, Unit::Meters), //real_area_sq_meters,
          Unit::Centimeters => UnitUtils::convert_area(real_area_sq, scale.unit, Unit::Centimeters), //real_area_sq_meters / (0.01 * 0.01),
        };
        Some(real_area)
      }
      _ => None,
    }
  }

  pub fn real_area(&self) -> Option<f64> {
    match self {
      Measurement::Area {
        scale,
        display_unit,
        ..
      } => self.real_area_with_target(scale, display_unit),
      _ => None,
    }
  }

  /// Calculate pixel perimeter for area measurements
  pub fn pixel_perimeter(&self) -> Option<f64> {
    match self {
      Measurement::Area {
        rectangle, polygon, ..
      } => {
        if let Some(rect) = rectangle {
          Some(rect.perimeter())
        } else if let Some(poly) = polygon {
          if poly.points.len() < 2 {
            Some(0.0)
          } else {
            Some(poly.perimeter())
          }
        } else {
          None
        }
      }
      _ => None,
    }
  }

  /// Calculate pixel length for linear measurements
  pub fn pixel_length(&self) -> Option<f64> {
    match self {
      Measurement::Linear { line, polyline, .. } => {
        if let Some(l) = line {
          Some(l.length())
        } else if let Some(poly) = polyline {
          Some(poly.length())
        } else {
          None
        }
      }
      _ => None,
    }
  }

  /// Get the display value as a string
  pub fn display_value(&self) -> String {
    match self {
      Measurement::Linear {
        scale,
        display_unit,
        ..
      } => {
        if let Some(pixel_length) = self.pixel_length() {
          let real_length = scale.pixel_to_real_unit(pixel_length, *display_unit);
          format!("{:.2} {}", real_length, display_unit.display())
        } else {
          "0.00".to_string()
        }
      }
      Measurement::Area { display_unit, .. } => {
        let real_area = self.real_area().unwrap_or(0.0);
        format!("{:.2} {}²", real_area, display_unit.display())
      }
      Measurement::Count { .. } => "1".to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::takeoff_core::{Point, Polygon, Rectangle, Scale, Unit};

  #[test]
  fn test_negative_area() {
    let polygon = Polygon::new(vec![
      Point::new(363.89389747057834, 622.4529406836895),
      Point::new(372.4963432958867, 836.5356779704213),
      Point::new(524.7039317219491, 837.3044031644923),
      Point::new(525.4726569160201, 634.3609519297424),
      Point::new(508.5607026464576, 616.6802724661089),
    ]);
    let scale = Scale::new(140.9233489, 76.0, Unit::Feet);

    let measurement = Measurement::Area {
      id: "poly1".to_string(),
      rectangle: None,
      polygon: Some(polygon),
      scale,
      display_unit: Unit::Feet,
    };

    let area = measurement.real_area().unwrap();
    assert_eq!(area.round(), 9894.0);
  }

  #[test]
  fn test_real_area_rectangle_scale_works() {
    // Rectangle: width=100, height=50 in pixels
    let rect = Rectangle {
      start: Point { x: 0.0, y: 0.0 },
      end: Point { x: 100.0, y: 50.0 },
    };

    // Scale: 100 pixels == 2 meters
    let scale = Scale::new(100.0, 2.0, Unit::Meters);

    let measurement = Measurement::Area {
      id: "rect1".to_string(),
      rectangle: Some(rect),
      polygon: None,
      scale,
      display_unit: Unit::Meters,
    };

    // Area in pixels: 100 * 50 = 5000
    // Each 100 pixels = 2 meters --> 1 pixel = 0.02 meters
    // So width = 2 meters, height = 1 meter, area = 2 * 1 = 2 square meters
    let pixel_area = rect.area();
    assert_eq!(pixel_area, 5000.0);

    let real_area = measurement.real_area();
    let expected_area = 2.0 * 1.0;
    assert!(real_area.is_some());
    let real_area_val = real_area.unwrap();

    // Allow for floating point error
    assert!((real_area_val - expected_area).abs() < 1e-10);

    // Test display value for metric unit
    assert!(measurement.display_value().starts_with("2.00 m²"));
  }

  #[test]
  fn test_real_area_polygon_scale_works() {
    // Simple triangle (right) with pixels: (0,0), (40,0), (0,30)
    let triangle_points = vec![
      Point { x: 0.0, y: 0.0 },
      Point { x: 40.0, y: 0.0 },
      Point { x: 0.0, y: 30.0 },
    ];
    let polygon = Polygon::new(triangle_points);

    // Area should be 0.5 * base * height = 0.5 * 40 * 30 = 600 px^2
    let pixel_area = polygon.area();
    assert_eq!(pixel_area.abs(), 600.0);

    // Scale: 20 pixels = 1 foot
    let scale = Scale::new(20.0, 1.0, Unit::Feet);

    let measurement = Measurement::Area {
      id: "poly1".to_string(),
      rectangle: None,
      polygon: Some(polygon),
      scale,
      display_unit: Unit::Feet,
    };

    // 20 pixels = 1 foot -> 1 pixel = 0.05 foot
    // So, 40 px = 2 ft, 30 px = 1.5 ft, area = 0.5*2*1.5 = 1.5 ft²
    let expected_area = 1.5;
    let real_area = measurement.real_area();
    assert!(real_area.is_some());
    let real_area_val = real_area.unwrap();
    assert!((real_area_val - expected_area).abs() < 1e-10);

    // Test display value for imperial unit
    assert!(measurement.display_value().starts_with("1.50 ft²"));
  }

  #[test]
  fn test_real_area_area_unit_conversion() {
    // Rectangle: 50 (w) x 40 (h) pixels
    let rect = Rectangle {
      start: Point { x: 0.0, y: 0.0 },
      end: Point { x: 50.0, y: 40.0 },
    };
    // Scale: 10 pixels = 1 meter
    let scale = Scale::new(10.0, 1.0, Unit::Meters);

    // Display in centimeters
    let measurement = Measurement::Area {
      id: "rectCm".to_string(),
      rectangle: Some(rect),
      polygon: None,
      scale,
      display_unit: Unit::Centimeters,
    };

    // Each 10 pixels = 1 meter, so 1 px = 0.1 m = 10 cm
    // So width = 5 m = 500 cm, height = 4 m = 400 cm, area = 500*400 = 200,000 cm²
    let expected_area_cm2 = 500.0 * 400.0;
    let real_area_cm2 = measurement.real_area().unwrap();
    assert!((real_area_cm2 - expected_area_cm2).abs() < 1e-6);

    // Display value
    assert!(measurement.display_value().starts_with("200000.00 cm²"));
  }

  #[test]
  fn test_real_area_conversion_precision() {
    let scale = Scale::new(142.98, 76.0, Unit::Feet);
    let ratio = scale.ratio();

    let start = Point {
      x: 546.85,
      y: 614.148,
    };
    let end = Point {
      x: start.x + (76.0 * ratio),
      y: start.y + (140.0 * ratio),
    };

    let rectangle = Rectangle { start, end };

    let rect_measurement = Measurement::Area {
      id: "rect".to_string(),
      rectangle: Some(rectangle),
      polygon: None,
      scale,
      display_unit: Unit::Feet,
    };

    let polygon = Polygon::new(vec![
      start,
      Point {
        x: end.x,
        y: start.y,
      },
      end,
      Point {
        x: start.x,
        y: end.y,
      },
    ]);

    let poly_measurement = Measurement::Area {
      id: "poly".to_string(),
      rectangle: None,
      polygon: Some(polygon),
      scale,
      display_unit: Unit::Feet,
    };

    let rect_pixel_area = rect_measurement.pixel_area().unwrap();
    let poly_pixel_area = poly_measurement.pixel_area().unwrap();
    assert!((rect_pixel_area - poly_pixel_area).abs() < 1e-9);

    let rect_real_area = rect_measurement.real_area().unwrap();
    let poly_real_area = poly_measurement.real_area().unwrap();
    assert!((rect_real_area - poly_real_area).abs() < 1e-9);

    assert!((rect_real_area - 10640.0).abs() < 1e-9);
    assert!((poly_real_area - 10640.0).abs() < 1e-9);
  }

  #[test]
  fn test_real_area_rectangle_and_polygon_are_equal() {
    let rect = Rectangle {
      start: Point { x: 0.0, y: 0.0 },
      end: Point { x: 100.0, y: 50.0 },
    };
    let polygon = Polygon::new(vec![
      Point { x: 0.0, y: 0.0 },
      Point { x: 100.0, y: 0.0 },
      Point { x: 100.0, y: 50.0 },
      Point { x: 0.0, y: 50.0 },
    ]);
    let scale = Scale::new(25.0, 2.0, Unit::Meters);
    let rect_measurement = Measurement::Area {
      id: "rect".to_string(),
      rectangle: Some(rect),
      // polygon: Some(polygon),
      polygon: None,
      scale,
      display_unit: Unit::Meters,
    };
    let poly_measurement = Measurement::Area {
      id: "poly".to_string(),
      rectangle: None,
      polygon: Some(polygon),
      scale,
      display_unit: Unit::Meters,
    };
    println!(
      "rect_measurement: {:?}",
      rect_measurement.real_area().unwrap()
    );
    println!(
      "poly_measurement: {:?}",
      poly_measurement.real_area().unwrap()
    );
    assert!(rect_measurement.pixel_area().unwrap() == poly_measurement.pixel_area().unwrap());
    assert!(rect_measurement.real_area().unwrap() == poly_measurement.real_area().unwrap());
  }

  #[test]
  fn test_real_area_returns_none_for_non_area() {
    let m = Measurement::Linear {
      id: "linear".to_string(),
      line: None,
      polyline: None,
      scale: Scale::new(100.0, 2.0, Unit::Meters),
      display_unit: Unit::Meters,
    };
    assert!(m.real_area().is_none());

    let m2 = Measurement::Count {
      id: "ct".to_string(),
      point: Point { x: 0.0, y: 0.0 },
    };
    assert!(m2.real_area().is_none());
  }

  #[test]
  fn test_real_area_no_shape_returns_none() {
    let scale = Scale::new(100.0, 2.0, Unit::Meters);
    let m = Measurement::Area {
      id: "empty".to_string(),
      rectangle: None,
      polygon: None,
      scale,
      display_unit: Unit::Meters,
    };
    assert!(m.real_area().unwrap() == 0.0);
  }
}
