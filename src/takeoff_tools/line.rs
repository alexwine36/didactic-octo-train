use crate::takeoff_core::{Line, Measurement, Point, Scale, Unit};
use crate::takeoff_tools::{TakeoffTool, ToolAction};

/// Tool for creating linear measurements with lines
pub struct LineTool {
  start_point: Option<Point>,
  end_point: Option<Point>,
  is_drawing: bool,
  scale: Option<Scale>,
  display_unit: Unit,
}

impl LineTool {
  pub fn new(scale: Option<Scale>, display_unit: Unit) -> Self {
    Self {
      start_point: None,
      end_point: None,
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
    format!("line_{}", uuid::Uuid::new_v4().to_string().replace('-', ""))
  }
}

impl TakeoffTool for LineTool {
  fn name(&self) -> &'static str {
    "line"
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
      if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
        let line = Line::new(start, end);
        let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
        ToolAction::UpdatePreview(Measurement::Linear {
          id: "preview".to_string(),
          line: Some(line),
          polyline: None,
          scale,
          display_unit: self.display_unit,
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
      if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
        let line = Line::new(start, end);
        let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
        let measurement = Measurement::Linear {
          id: Self::generate_id(),
          line: Some(line),
          polyline: None,
          scale,
          display_unit: self.display_unit,
        };
        self.start_point = None;
        self.end_point = None;
        self.is_drawing = false;
        ToolAction::CreateMeasurement(measurement)
      } else {
        ToolAction::None
      }
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
