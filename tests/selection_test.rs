use napi_test::takeoff_core::{Line, Measurement, Point, Rectangle, Scale, Transform, Unit};
use napi_test::takeoff_tools::SelectionTool;

#[test]
fn test_find_selection_linear_line() {
  let tool = SelectionTool::new(10.0);
  let transform = Transform::identity();

  let measurement = Measurement::Linear {
    id: "test1".to_string(),
    line: Some(Line::new(Point::new(10.0, 10.0), Point::new(50.0, 50.0))),
    polyline: None,
    scale: Scale::new(1.0, 1.0, Unit::Meters),
    display_unit: Unit::Meters,
  };

  let measurements = vec![measurement];

  // Test selecting start vertex
  let result = tool.find_selection(Point::new(10.0, 10.0), &measurements, transform);
  assert!(result.is_some());
  let selection = result.unwrap();
  assert_eq!(selection.measurement_id, "test1");
  assert_eq!(selection.vertex_index, Some(0));

  // Test selecting end vertex
  let result = tool.find_selection(Point::new(50.0, 50.0), &measurements, transform);
  assert!(result.is_some());
  let selection = result.unwrap();
  assert_eq!(selection.measurement_id, "test1");
  assert_eq!(selection.vertex_index, Some(1));
}

#[test]
fn test_find_selection_area_rectangle() {
  let tool = SelectionTool::new(10.0);
  let transform = Transform::identity();

  let measurement = Measurement::Area {
    id: "test2".to_string(),
    rectangle: Some(Rectangle::new(
      Point::new(0.0, 0.0),
      Point::new(100.0, 100.0),
    )),
    polygon: None,
    scale: Scale::new(1.0, 1.0, Unit::Meters),
    display_unit: Unit::Meters,
  };

  let measurements = vec![measurement];

  // Test selecting start vertex
  let result = tool.find_selection(Point::new(0.0, 0.0), &measurements, transform);
  assert!(result.is_some());
  let selection = result.unwrap();
  assert_eq!(selection.measurement_id, "test2");
  assert_eq!(selection.vertex_index, Some(0));
}

#[test]
fn test_update_vertex_linear_line() {
  let tool = SelectionTool::new(10.0);
  let mut measurement = Measurement::Linear {
    id: "test3".to_string(),
    line: Some(Line::new(Point::new(10.0, 10.0), Point::new(50.0, 50.0))),
    polyline: None,
    scale: Scale::new(1.0, 1.0, Unit::Meters),
    display_unit: Unit::Meters,
  };

  // Update start vertex
  let updated = tool.update_vertex(&mut measurement, 0, Point::new(20.0, 20.0));
  assert!(updated);

  if let Measurement::Linear { line, .. } = measurement {
    if let Some(l) = line {
      assert_eq!(l.start, Point::new(20.0, 20.0));
      assert_eq!(l.end, Point::new(50.0, 50.0));
    } else {
      panic!("Line should exist");
    }
  } else {
    panic!("Measurement should be Linear");
  }
}

#[test]
fn test_get_measurement_vertices() {
  let tool = SelectionTool::new(10.0);
  let measurement = Measurement::Linear {
    id: "test4".to_string(),
    line: Some(Line::new(Point::new(10.0, 10.0), Point::new(50.0, 50.0))),
    polyline: None,
    scale: Scale::new(1.0, 1.0, Unit::Meters),
    display_unit: Unit::Meters,
  };

  let vertices = tool.get_measurement_vertices(&measurement);
  assert_eq!(vertices.len(), 2);
  assert_eq!(vertices[0], Point::new(10.0, 10.0));
  assert_eq!(vertices[1], Point::new(50.0, 50.0));
}
