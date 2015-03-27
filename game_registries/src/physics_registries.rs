extern crate cgmath;
use std::collections::HashMap;

pub use cgmath::Vector3;

mod physics_registries {
  pub struct PositionComponent {
    pub pos: Vector3<f32>
  }

  impl PositionComponent {
    pub fn new(x: f32, y: f32, z: f32) -> PositionComponent {
      PositionComponent { pos: Vector3::new(x, y, z) }
    }

    pub fn from_vec3(pos: Vector3<f32>) -> PositionComponent {
      PositionComponent { pos: pos }
    }
  }

  pub struct VelocityComponent {
    pub vel: Vector3<f32>
  }

  impl VelocityComponent {
    pub fn new(x: f32, y: f32, z: f32) -> VelocityComponent {
      VelocityComponent { vel: Vector3::new(x, y, z) }
    }

    pub fn from_vec3(vel: Vector3<f32>) -> VelocityComponent {
      VelocityComponent { vel: vel }
    }
  }

  pub struct MassComponent {
    pub mass: f32
  }

  impl MassComponent {
    pub fn new(mass: f32) -> MassComponent {
      MassComponent { mass: mass }
    }
  }

