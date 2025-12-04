use crate::takeoff_core::{Line, Measurement, Point, Scale, Unit};
use crate::takeoff_tools::{TakeoffTool, ToolAction};

/// Tool for setting the scale by drawing a reference line
pub struct ScaleTool {
  start_point: Option<Point>,
  end_point: Option<Point>,
  is_drawing: bool,
}

impl ScaleTool {
  pub fn new() -> Self {
    Self {
      start_point: None,
      end_point: None,
      is_drawing: false,
    }
  }

  /// Create a scale from the drawn line and real-world distance
  pub fn create_scale(&self, real_distance: f64, unit: Unit) -> Option<Scale> {
    if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
      let line = Line::new(start, end);
      let pixel_distance = line.length();
      Some(Scale::new(pixel_distance, real_distance, unit))
    } else {
      None
    }
  }

  /// Get the current line being drawn
  pub fn current_line(&self) -> Option<Line> {
    if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
      Some(Line::new(start, end))
    } else {
      None
    }
  }
}

impl Default for ScaleTool {
  fn default() -> Self {
    Self::new()
  }
}

impl TakeoffTool for ScaleTool {
  fn name(&self) -> &'static str {
    "scale"
  }

  fn on_mouse_down(&mut self, point: Point) -> ToolAction {
    if self.start_point.is_none() {
      self.start_point = Some(point);
      self.is_drawing = true;
      ToolAction::None
    } else {
      ToolAction::None
    }
  }

  fn on_mouse_move(&mut self, point: Point) -> ToolAction {
    if self.is_drawing {
      self.end_point = Some(point);
      if let Some(line) = self.current_line() {
        // Create a temporary measurement for preview
        // Scale tool doesn't create a final measurement until scale is set
        ToolAction::UpdatePreview(Measurement::Linear {
          id: "preview".to_string(),
          line: Some(line),
          polyline: None,
          scale: Scale::new(1.0, 1.0, Unit::Meters), // Dummy scale for preview
          display_unit: Unit::Meters,
        })
      } else {
        ToolAction::None
      }
    } else {
      ToolAction::None
    }
  }

  fn on_mouse_up(&mut self, point: Point) -> ToolAction {
    if self.is_drawing {
      self.end_point = Some(point);
      // Scale is set via create_scale() method, not here
      // Return cancel to indicate drawing is complete but no measurement yet
      ToolAction::Cancel
    } else {
      ToolAction::None
    }
  }

  fn cancel(&mut self) -> ToolAction {
    self.start_point = None;
    self.end_point = None;
    self.is_drawing = false;
    ToolAction::Cancel
  }

  fn is_drawing(&self) -> bool {
    self.is_drawing
  }
}
