use napi_test::takeoff_core::*;

#[test]
fn test_point_distance() {
  let p1 = Point::new(0.0, 0.0);
  let p2 = Point::new(3.0, 4.0);
  assert_eq!(p1.distance_to(&p2), 5.0);
}

#[test]
fn test_rectangle_area() {
  let rect = Rectangle::new(Point::new(0.0, 0.0), Point::new(10.0, 5.0));
  assert_eq!(rect.width(), 10.0);
  assert_eq!(rect.height(), 5.0);
  assert_eq!(rect.area(), 50.0);
}

#[test]
fn test_rectangle_perimeter() {
  let rect = Rectangle::new(Point::new(0.0, 0.0), Point::new(10.0, 5.0));
  assert_eq!(rect.perimeter(), 30.0);
}

#[test]
fn test_polygon_area() {
  let polygon = Measurement::Area {
    id: "test".to_string(),
    rectangle: None,
    polygon: Some(Polygon::new(vec![
      Point::new(0.0, 0.0),
      Point::new(10.0, 0.0),
      Point::new(10.0, 5.0),
      Point::new(0.0, 5.0),
    ])),
    scale: Scale::new(1.0, 1.0, Unit::Meters),
    display_unit: Unit::Feet,
  };
  let area = polygon.pixel_area().unwrap();
  assert_eq!(area, 50.0);
}

#[test]
fn test_calculate_irregular_polygon_area() {
  let polygon = Measurement::Area {
    id: "test".to_string(),
    rectangle: None,
    polygon: Some(Polygon::new(vec![
      Point::new(0.0, 0.0),
      Point::new(5.0, 5.0),
      Point::new(10.0, 0.0),
      Point::new(10.0, 10.0),
      Point::new(5.0, 15.0),
      Point::new(0.0, 10.0),
    ])),
    scale: Scale::new(1.0, 1.0, Unit::Meters),
    display_unit: Unit::Feet,
  };
  let area = polygon.pixel_area().unwrap();
  assert_eq!(area, 100.0);
}

#[test]
fn test_polygon_perimeter() {
  let polygon = Measurement::Area {
    id: "test".to_string(),
    rectangle: None,
    polygon: Some(Polygon::new(vec![
      Point::new(0.0, 0.0),
      Point::new(10.0, 0.0),
      Point::new(10.0, 5.0),
      Point::new(0.0, 5.0),
    ])),
    scale: Scale::new(1.0, 1.0, Unit::Meters),
    display_unit: Unit::Feet,
  };
  let perimeter = polygon.pixel_perimeter().unwrap();
  assert_eq!(perimeter, 30.0);
}

#[test]
fn test_line_length() {
  let line = Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
  assert_eq!(line.length(), 5.0);
}

#[test]
fn test_unit_conversion() {
  // Test feet to meters
  let feet = Unit::Feet;
  let meters = Unit::Meters;
  let value = 10.0;
  let converted = feet.convert(value, &meters);
  assert!((converted - 3.048).abs() < 0.001);
}

#[test]
fn test_scale_conversion() {
  let scale = Scale::new(100.0, 10.0, Unit::Feet);
  let pixel_distance = 50.0;
  let real_distance = scale.pixel_to_real(pixel_distance);
  assert!((real_distance - 5.0).abs() < 0.001);
}

#[test]
fn test_takeoff_state() {
  let mut state = TakeoffState::new();
  assert_eq!(state.measurements.len(), 0);

  let measurement = Measurement::Count {
    id: "test1".to_string(),
    point: Point::new(10.0, 20.0),
  };

  state.add_measurement(measurement);
  assert_eq!(state.measurements.len(), 1);
  assert_eq!(state.count, 1);

  state.remove_measurement("test1");
  assert_eq!(state.measurements.len(), 0);
  assert_eq!(state.count, 0);
}

#[test]
fn test_transform() {
  let transform = Transform::new(2.0, 10.0, 20.0);
  let world_point = Point::new(5.0, 10.0);
  let screen_point = transform.world_to_screen(world_point);

  assert_eq!(screen_point.x, 20.0);
  assert_eq!(screen_point.y, 40.0);

  let back_to_world = transform.screen_to_world(screen_point);
  assert!((back_to_world.x - 5.0).abs() < 0.001);
  assert!((back_to_world.y - 10.0).abs() < 0.001);
}
