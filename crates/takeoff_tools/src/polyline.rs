use crate::{TakeoffTool, ToolAction};
use takeoff_core::{Measurement, Point, Polyline, Scale, Unit};

/// Tool for creating linear measurements with polylines (multiple connected line segments)
pub struct PolylineTool {
  points: Vec<Point>,
  is_drawing: bool,
  scale: Option<Scale>,
  display_unit: Unit,
}

impl PolylineTool {
  pub fn new(scale: Option<Scale>, display_unit: Unit) -> Self {
    Self {
      points: Vec::new(),
      is_drawing: false,
      scale,
      display_unit,
    }
  }

  pub fn set_scale(&mut self, scale: Option<Scale>) {
    self.scale = scale;
  }

  pub fn set_display_unit(&mut self, unit: Unit) {
    self.display_unit = unit;
  }

  fn generate_id() -> String {
    format!(
      "polyline_{}",
      uuid::Uuid::new_v4().to_string().replace('-', "")
    )
  }
}

impl TakeoffTool for PolylineTool {
  fn name(&self) -> &'static str {
    "polyline"
  }

  fn on_mouse_down(&mut self, point: Point) -> ToolAction {
    if !self.is_drawing {
      // First point - start drawing
      self.points.push(point);
      self.is_drawing = true;
      ToolAction::None
    } else {
      // Add new point
      self.points.push(point);
      // Update preview - show polyline so far plus line to current point
      let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
      ToolAction::UpdatePreview(Measurement::Linear {
        id: "preview".to_string(),
        line: None,
        polyline: Some(Polyline::new(self.points.clone())),
        scale,
        display_unit: self.display_unit,
      })
    }
  }

  fn on_mouse_move(&mut self, _point: Point) -> ToolAction {
    if self.is_drawing && !self.points.is_empty() {
      // Update preview with polyline so far
      let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
      ToolAction::UpdatePreview(Measurement::Linear {
        id: "preview".to_string(),
        line: None,
        polyline: Some(Polyline::new(self.points.clone())),
        scale,
        display_unit: self.display_unit,
      })
    } else {
      ToolAction::None
    }
  }

  fn on_mouse_up(&mut self, _point: Point) -> ToolAction {
    // Polyline handles clicks in mouse_down, not mouse_up
    ToolAction::None
  }

  fn cancel(&mut self) -> ToolAction {
    if self.points.len() >= 2 {
      // Finish the polyline with current points
      let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
      let measurement = Measurement::Linear {
        id: Self::generate_id(),
        line: None,
        polyline: Some(Polyline::new(self.points.clone())),
        scale,
        display_unit: self.display_unit,
      };
      self.points.clear();
      self.is_drawing = false;
      ToolAction::CreateMeasurement(measurement)
    } else {
      self.points.clear();
      self.is_drawing = false;
      ToolAction::Cancel
    }
  }

  fn is_drawing(&self) -> bool {
    self.is_drawing
  }
}
