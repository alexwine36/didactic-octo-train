pub mod count;
pub mod line;
pub mod polygon;
pub mod polyline;
pub mod rectangle;
pub mod scale;
pub mod selection;

pub use count::CountTool;
pub use line::LineTool;
pub use polygon::PolygonTool;
pub use polyline::PolylineTool;
pub use rectangle::RectangleTool;
pub use scale::ScaleTool;
pub use selection::{SelectionResult, SelectionTool};

use takeoff_core::{Measurement, Point};

/// Trait for takeoff tools that can create measurements
pub trait TakeoffTool {
  /// Get the name of the tool
  fn name(&self) -> &'static str;

  /// Handle mouse down event
  fn on_mouse_down(&mut self, point: Point) -> ToolAction;

  /// Handle mouse move event
  fn on_mouse_move(&mut self, point: Point) -> ToolAction;

  /// Handle mouse up event
  fn on_mouse_up(&mut self, point: Point) -> ToolAction;

  /// Cancel the current operation
  fn cancel(&mut self) -> ToolAction;

  /// Check if the tool is currently drawing
  fn is_drawing(&self) -> bool;
}

/// Actions that tools can return
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ToolAction {
  /// No action needed
  #[default]
  None,
  /// Create a new measurement
  CreateMeasurement(Measurement),
  /// Update preview (for drawing operations)
  UpdatePreview(Measurement),
  /// Cancel current operation
  Cancel,
}
