use crate::takeoff_core::{Measurement, Point, Polygon, Scale, Unit};
use crate::takeoff_tools::{TakeoffTool, ToolAction};

/// Tool for creating area measurements with polygons
pub struct PolygonTool {
  points: Vec<Point>,
  is_drawing: bool,
  scale: Option<Scale>,
  display_unit: Unit,
  close_threshold: f64,
}

impl PolygonTool {
  pub fn new(scale: Option<Scale>, display_unit: Unit) -> Self {
    Self {
      points: Vec::new(),
      is_drawing: false,
      scale,
      display_unit,
      close_threshold: 10.0,
    }
  }

  pub fn set_scale(&mut self, scale: Option<Scale>) {
    self.scale = scale;
  }

  pub fn set_display_unit(&mut self, unit: Unit) {
    self.display_unit = unit;
  }

  pub fn set_close_threshold(&mut self, threshold: f64) {
    self.close_threshold = threshold;
  }

  fn generate_id() -> String {
    format!(
      "polygon_{}",
      uuid::Uuid::new_v4().to_string().replace('-', "")
    )
  }

  /// Check if a point is close enough to the start point to close the polygon
  fn is_close_to_start(&self, point: &Point) -> bool {
    if self.points.is_empty() {
      return false;
    }
    let start = &self.points[0];
    start.distance_to(point) < self.close_threshold
  }
}

impl TakeoffTool for PolygonTool {
  fn name(&self) -> &'static str {
    "polygon"
  }

  fn on_mouse_down(&mut self, point: Point) -> ToolAction {
    if self.points.is_empty() {
      // First point - start drawing
      self.points.push(point);
      self.is_drawing = true;
      ToolAction::None
    } else if self.is_close_to_start(&point) && self.points.len() >= 2 {
      // Close polygon by clicking near start
      if self.points.len() >= 3 {
        let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
        let measurement = Measurement::Area {
          id: Self::generate_id(),
          polygon: Some(Polygon::new(self.points.clone())),
          rectangle: None,
          scale,
          display_unit: self.display_unit,
        };
        self.points.clear();
        self.is_drawing = false;
        ToolAction::CreateMeasurement(measurement)
      } else {
        ToolAction::None
      }
    } else {
      // Add new point
      self.points.push(point);
      // Update preview
      let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
      ToolAction::UpdatePreview(Measurement::Area {
        id: "preview".to_string(),
        polygon: Some(Polygon::new(self.points.clone())),
        rectangle: None,
        scale,
        display_unit: self.display_unit,
      })
    }
  }

  fn on_mouse_move(&mut self, point: Point) -> ToolAction {
    if self.is_drawing && !self.points.is_empty() {
      // Update preview with current hover point
      let mut preview_points = self.points.clone();
      preview_points.push(point);
      let scale = self.scale.unwrap_or(Scale::new(1.0, 1.0, Unit::Meters));
      ToolAction::UpdatePreview(Measurement::Area {
        id: "preview".to_string(),
        polygon: Some(Polygon::new(preview_points)),
        rectangle: None,
        scale,
        display_unit: self.display_unit,
      })
    } else {
      ToolAction::None
    }
  }

  fn on_mouse_up(&mut self, _point: Point) -> ToolAction {
    // Polygon handles clicks in mouse_down, not mouse_up
    ToolAction::None
  }

  fn cancel(&mut self) -> ToolAction {
    self.points.clear();
    self.is_drawing = false;
    ToolAction::Cancel
  }

  fn is_drawing(&self) -> bool {
    self.is_drawing
  }
}
