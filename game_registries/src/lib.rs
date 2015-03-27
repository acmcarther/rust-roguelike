pub mod physics_registries;
pub mod control_registries;

use geometry_registries::*;
use input_registries::*;
use physics_registries::*;

pub type PositionRegistry      = HashMap<usize, PositionComponent>;
pub type VelocityRegistry      = HashMap<usize, VelocityComponent>;
pub type MassRegistry          = HashMap<usize, MassComponent>;
pub type DynamicVertexRegistry = HashMap<usize, DynamicVertexComponent>;
pub type ShapeRegistry         = HashMap<usize, ShapeComponent>;
pub type MeshComponent         = HashMap<usize, MeshComponent>;
pub type KeyboardRegistry      = HashMap<usize, KeyboardComponent>

