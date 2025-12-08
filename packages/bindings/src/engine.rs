use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::{
  sync::{Arc, Mutex},
  thread,
};
use takeoff_core::{Point, Scale, TakeoffState, Transform, Unit};

#[napi]
#[derive(Serialize, Deserialize, Clone)]
pub struct TakeoffEngine {
  state: TakeoffState,
  transform: Transform,
  scale: Option<Scale>,
  display_unit: Unit,

  #[serde(skip)]
  transform_callbacks: Arc<Mutex<Vec<ThreadsafeFunction<Transform>>>>,
}

#[napi]
impl TakeoffEngine {
  #[napi(constructor)]
  pub fn new(transform: Transform, scale: Option<Scale>, display_unit: Option<Unit>) -> Self {
    Self {
      state: TakeoffState::new(),
      transform,
      scale,
      display_unit: display_unit.unwrap_or(Unit::Feet),
      transform_callbacks: Arc::new(Mutex::new(Vec::new())),
    }
  }

  #[napi(getter)]
  pub fn display_unit(&self) -> Unit {
    self.display_unit.clone()
  }
  #[napi(setter)]
  pub fn set_display_unit(&mut self, display_unit: Unit) {
    self.display_unit = display_unit;
  }

  #[napi(getter)]
  pub fn state(&self) -> TakeoffState {
    self.state.clone()
  }
  #[napi(setter)]
  pub fn set_state(&mut self, state: TakeoffState) {
    self.state = state;
  }

  #[napi(getter)]
  pub fn transform(&self) -> Transform {
    self.transform.clone()
  }

  fn call_transform_callbacks(&self, transform: Transform) {
    let transform_callbacks = self.transform_callbacks.clone();
    thread::spawn(move || {
      for callback in transform_callbacks.lock().unwrap().iter() {
        let _ = callback.call(Ok(transform), ThreadsafeFunctionCallMode::Blocking);
      }
    });
  }

  #[napi(setter)]
  pub fn set_transform(&mut self, transform: Transform) {
    self.transform = transform;
    self.call_transform_callbacks(transform);
  }

  #[napi(getter)]
  pub fn scale(&self) -> Option<Scale> {
    self.scale.clone()
  }
  #[napi(setter)]
  pub fn set_scale(&mut self, scale: Either<Scale, Null>) {
    self.scale = match scale {
      Either::A(scale) => Some(scale),
      Either::B(_) => None,
    };
  }

  #[napi]
  pub fn add_transform_callback(&mut self, callback: ThreadsafeFunction<Transform>) {
    self.transform_callbacks.lock().unwrap().push(callback);
  }
}

/// Coordinate transformation methods
#[napi]
impl TakeoffEngine {
  /// Transform a point from screen coordinates to world coordinates
  #[napi]
  pub fn screen_to_world(&self, point: Point) -> Point {
    self.transform.screen_to_world(point)
  }

  /// Transform a point from world coordinates to screen coordinates
  #[napi]
  pub fn world_to_screen(&self, point: Point) -> Point {
    self.transform.world_to_screen(point)
  }

  /// Apply zoom transformation around a specific point
  #[napi]
  pub fn zoom_around_point(&mut self, center: Point, zoom_factor: f64) -> Transform {
    let world_center = self.transform.screen_to_world(center);

    let new_scale = (self.transform.scale * zoom_factor).clamp(0.1, 10.0);

    // Calculate new offset to keep the center point fixed
    let new_offset_x = center.x - world_center.x * new_scale;
    let new_offset_y = center.y - world_center.y * new_scale;

    self.transform = Transform::new(new_scale, new_offset_x, new_offset_y);
    self.transform
  }

  /// Apply pan transformation
  #[napi]
  pub fn pan(&mut self, delta_x: f64, delta_y: f64) -> Transform {
    self.set_transform(Transform::new(
      self.transform.scale,
      self.transform.offset_x + delta_x,
      self.transform.offset_y + delta_y,
    ));
    self.transform
  }

  /// Reset transform to identity
  #[napi]
  pub fn reset_transform(&mut self) -> Transform {
    self.transform = Transform::identity();
    self.transform
  }

  /// Fit image to viewport
  #[napi]
  pub fn fit_to_viewport(
    &mut self,
    image_width: f64,
    image_height: f64,
    viewport_width: f64,
    viewport_height: f64,
  ) -> Transform {
    let scale_x = viewport_width / image_width;
    let scale_y = viewport_height / image_height;
    let scale = scale_x.min(scale_y).min(1.0); // Don't scale up beyond 1.0

    let scaled_width = image_width * scale;
    let scaled_height = image_height * scale;

    let offset_x = (viewport_width - scaled_width) / 2.0;
    let offset_y = (viewport_height - scaled_height) / 2.0;

    self.transform = Transform::new(scale, offset_x, offset_y);
    self.transform
  }
}
