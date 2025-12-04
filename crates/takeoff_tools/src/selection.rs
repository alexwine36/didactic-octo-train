use takeoff_core::{Measurement, Point, Transform};
use crate::{TakeoffTool, ToolAction};

/// Result of a selection operation
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionResult {
  /// ID of the selected measurement
  pub measurement_id: String,
  /// Index of the selected vertex (None if measurement selected but no vertex)
  pub vertex_index: Option<usize>,
  /// The point that was selected
  pub point: Point,
}

/// Tool for selecting and editing measurements
pub struct SelectionTool {
  /// Threshold distance for selecting vertices (in world coordinates)
  vertex_threshold: f64,
  /// Currently selected measurement ID
  selected_measurement_id: Option<String>,
  /// Currently selected vertex index
  selected_vertex_index: Option<usize>,
  /// Whether we're currently dragging a vertex
  is_dragging: bool,
  /// The point where dragging started
  drag_start: Option<Point>,
}

impl SelectionTool {
  pub fn new(vertex_threshold: f64) -> Self {
    Self {
      vertex_threshold,
      selected_measurement_id: None,
      selected_vertex_index: None,
      is_dragging: false,
      drag_start: None,
    }
  }

  pub fn set_vertex_threshold(&mut self, threshold: f64) {
    self.vertex_threshold = threshold;
  }

  /// Find the nearest vertex to a point within the threshold
  fn find_nearest_vertex(
    &self,
    point: Point,
    measurement: &Measurement,
    threshold: f64,
  ) -> Option<usize> {
    let vertices = self.get_measurement_vertices(measurement);

    for (index, vertex) in vertices.iter().enumerate() {
      if vertex.distance_to(&point) <= threshold {
        return Some(index);
      }
    }
    None
  }

  /// Get all vertices for a measurement
  pub fn get_measurement_vertices(&self, measurement: &Measurement) -> Vec<Point> {
    match measurement {
      Measurement::Linear { line, polyline, .. } => {
        if let Some(l) = line {
          vec![l.start, l.end]
        } else if let Some(points) = polyline {
          points.points.clone()
        } else {
          Vec::new()
        }
      }
      Measurement::Area {
        rectangle, polygon, ..
      } => {
        if let Some(rect) = rectangle {
          vec![rect.start, rect.end]
        } else if let Some(points) = polygon {
          points.points.clone()
        } else {
          Vec::new()
        }
      }
      Measurement::Count { point, .. } => {
        vec![*point]
      }
    }
  }

  /// Get a specific vertex from a measurement
  pub fn get_vertex(&self, measurement: &Measurement, index: usize) -> Option<Point> {
    let vertices = self.get_measurement_vertices(measurement);
    vertices.get(index).copied()
  }

  /// Update a vertex in a measurement
  pub fn update_vertex(
    &self,
    measurement: &mut Measurement,
    vertex_index: usize,
    new_point: Point,
  ) -> bool {
    match measurement {
      Measurement::Linear { line, polyline, .. } => {
        if let Some(l) = line {
          match vertex_index {
            0 => l.start = new_point,
            1 => l.end = new_point,
            _ => return false,
          }
          true
        } else if let Some(poly) = polyline {
          if let Some(point) = poly.points.get_mut(vertex_index) {
            *point = new_point;
            true
          } else {
            false
          }
        } else {
          false
        }
      }
      Measurement::Area {
        rectangle, polygon, ..
      } => {
        if let Some(rect) = rectangle {
          match vertex_index {
            0 => rect.start = new_point,
            1 => rect.end = new_point,
            _ => return false,
          }
          true
        } else if let Some(poly) = polygon {
          if let Some(point) = poly.points.get_mut(vertex_index) {
            *point = new_point;
            true
          } else {
            false
          }
        } else {
          false
        }
      }
      Measurement::Count { point, .. } => {
        if vertex_index == 0 {
          *point = new_point;
          true
        } else {
          false
        }
      }
    }
  }

  /// Find the measurement and vertex at a given point
  pub fn find_selection(
    &self,
    point: Point,
    measurements: &[Measurement],
    transform: Transform,
  ) -> Option<SelectionResult> {
    // Convert point from screen to world coordinates if needed
    // For now, assume point is in world coordinates
    let world_point = point;

    // Check each measurement for nearby vertices
    for measurement in measurements {
      let vertex_index = self.find_nearest_vertex(
        world_point,
        measurement,
        self.vertex_threshold / transform.scale, // Adjust threshold for zoom
      );

      if let Some(index) = vertex_index {
        return Some(SelectionResult {
          measurement_id: measurement.id().to_string(),
          vertex_index: Some(index),
          point: world_point,
        });
      }
    }

    None
  }

  pub fn get_selected_measurement_id(&self) -> Option<&String> {
    self.selected_measurement_id.as_ref()
  }

  pub fn get_selected_vertex_index(&self) -> Option<usize> {
    self.selected_vertex_index
  }

  pub fn clear_selection(&mut self) {
    self.selected_measurement_id = None;
    self.selected_vertex_index = None;
  }
}

impl TakeoffTool for SelectionTool {
  fn name(&self) -> &'static str {
    "selection"
  }

  fn on_mouse_down(&mut self, point: Point) -> ToolAction {
    // Selection is handled by the frontend, which calls find_selection
    // This method is here to satisfy the trait but won't be used directly
    self.drag_start = Some(point);
    self.is_dragging = false;
    ToolAction::None
  }

  fn on_mouse_move(&mut self, _point: Point) -> ToolAction {
    if self.drag_start.is_some() {
      self.is_dragging = true;
    }
    ToolAction::None
  }

  fn on_mouse_up(&mut self, _point: Point) -> ToolAction {
    self.drag_start = None;
    self.is_dragging = false;
    ToolAction::None
  }

  fn cancel(&mut self) -> ToolAction {
    self.clear_selection();
    self.drag_start = None;
    self.is_dragging = false;
    ToolAction::Cancel
  }

  fn is_drawing(&self) -> bool {
    self.is_dragging
  }
}
