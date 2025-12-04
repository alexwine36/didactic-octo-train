use crate::{TakeoffTool, ToolAction};
use takeoff_core::{Measurement, Point};

/// Tool for creating count markers
pub struct CountTool {
  is_placing: bool,
}

impl CountTool {
  pub fn new() -> Self {
    Self { is_placing: false }
  }

  fn generate_id() -> String {
    format!(
      "count_{}",
      uuid::Uuid::new_v4().to_string().replace('-', "")
    )
  }
}

impl Default for CountTool {
  fn default() -> Self {
    Self::new()
  }
}

impl TakeoffTool for CountTool {
  fn name(&self) -> &'static str {
    "count"
  }

  fn on_mouse_down(&mut self, _point: Point) -> ToolAction {
    self.is_placing = true;
    ToolAction::None
  }

  fn on_mouse_move(&mut self, _point: Point) -> ToolAction {
    ToolAction::None
  }

  fn on_mouse_up(&mut self, point: Point) -> ToolAction {
    if self.is_placing {
      let measurement = Measurement::Count {
        id: Self::generate_id(),
        point,
      };
      self.is_placing = false;
      ToolAction::CreateMeasurement(measurement)
    } else {
      ToolAction::None
    }
  }

  fn cancel(&mut self) -> ToolAction {
    self.is_placing = false;
    ToolAction::Cancel
  }

  fn is_drawing(&self) -> bool {
    self.is_placing
  }
}
