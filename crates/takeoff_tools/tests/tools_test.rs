use takeoff_core::{Point, Unit};
use takeoff_tools::*;

#[test]
fn test_scale_tool() {
  let mut tool = ScaleTool::new();
  assert_eq!(tool.name(), "scale");
  assert!(!tool.is_drawing());

  let _action = tool.on_mouse_down(Point::new(0.0, 0.0));
  assert!(tool.is_drawing());

  let action = tool.on_mouse_move(Point::new(10.0, 0.0));
  assert!(matches!(action, ToolAction::UpdatePreview(_)));

  let scale = tool.create_scale(5.0, Unit::Feet);
  assert!(scale.is_some());
  let scale = scale.unwrap();
  assert_eq!(scale.pixel_distance, 10.0);
  assert_eq!(scale.real_distance, 5.0);
}

#[test]
fn test_line_tool() {
  let mut tool = LineTool::new(None, Unit::Meters);
  assert_eq!(tool.name(), "line");

  tool.on_mouse_down(Point::new(0.0, 0.0));
  assert!(tool.is_drawing());

  let action = tool.on_mouse_move(Point::new(10.0, 0.0));
  assert!(matches!(action, ToolAction::UpdatePreview(_)));

  let action = tool.on_mouse_up(Point::new(10.0, 0.0));
  assert!(matches!(action, ToolAction::CreateMeasurement(_)));
  assert!(!tool.is_drawing());
}

#[test]
fn test_rectangle_tool() {
  let mut tool = RectangleTool::new(None, Unit::Meters);
  assert_eq!(tool.name(), "rectangle");

  tool.on_mouse_down(Point::new(0.0, 0.0));
  assert!(tool.is_drawing());

  let action = tool.on_mouse_move(Point::new(10.0, 5.0));
  assert!(matches!(action, ToolAction::UpdatePreview(_)));

  let action = tool.on_mouse_up(Point::new(10.0, 5.0));
  assert!(matches!(action, ToolAction::CreateMeasurement(_)));
  assert!(!tool.is_drawing());
}

#[test]
fn test_count_tool() {
  let mut tool = CountTool::new();
  assert_eq!(tool.name(), "count");

  tool.on_mouse_down(Point::new(10.0, 20.0));
  assert!(tool.is_drawing());

  let action = tool.on_mouse_up(Point::new(10.0, 20.0));
  assert!(matches!(action, ToolAction::CreateMeasurement(_)));
  assert!(!tool.is_drawing());
}

#[test]
fn test_polygon_tool() {
  let mut tool = PolygonTool::new(None, Unit::Meters);
  assert_eq!(tool.name(), "polygon");
  assert!(!tool.is_drawing());

  // First click - start polygon
  let action = tool.on_mouse_down(Point::new(0.0, 0.0));
  assert!(tool.is_drawing());
  assert!(matches!(action, ToolAction::None));

  // Second click - add point
  let action = tool.on_mouse_down(Point::new(10.0, 0.0));
  assert!(matches!(action, ToolAction::UpdatePreview(_)));

  // Third click - add another point
  let action = tool.on_mouse_down(Point::new(10.0, 10.0));
  assert!(matches!(action, ToolAction::UpdatePreview(_)));

  // Fourth click - close polygon by clicking near start
  let action = tool.on_mouse_down(Point::new(0.1, 0.1));
  assert!(matches!(action, ToolAction::CreateMeasurement(_)));
  assert!(!tool.is_drawing());
}

#[test]
fn test_polygon_tool_close_threshold() {
  let mut tool = PolygonTool::new(None, Unit::Meters);
  tool.set_close_threshold(5.0);

  tool.on_mouse_down(Point::new(0.0, 0.0));
  tool.on_mouse_down(Point::new(10.0, 0.0));
  tool.on_mouse_down(Point::new(10.0, 10.0));

  // Click within threshold (5.0) of start point
  let action = tool.on_mouse_down(Point::new(3.0, 3.0));
  assert!(matches!(action, ToolAction::CreateMeasurement(_)));
  assert!(!tool.is_drawing());
}

#[test]
fn test_polygon_tool_cancel() {
  let mut tool = PolygonTool::new(None, Unit::Meters);
  tool.on_mouse_down(Point::new(0.0, 0.0));
  tool.on_mouse_down(Point::new(10.0, 0.0));
  assert!(tool.is_drawing());

  let action = tool.cancel();
  assert!(matches!(action, ToolAction::Cancel));
  assert!(!tool.is_drawing());
}

#[test]
fn test_tool_cancel() {
  let mut scale_tool = ScaleTool::new();
  scale_tool.on_mouse_down(Point::new(0.0, 0.0));
  assert!(scale_tool.is_drawing());

  let action = scale_tool.cancel();
  assert!(matches!(action, ToolAction::Cancel));
  assert!(!scale_tool.is_drawing());
}
