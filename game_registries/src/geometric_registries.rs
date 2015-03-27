mod geometric_registries {
  pub struct EdgesComponent {
    vertex_count: usize
  }

  impl EdgesComponent {
    pub fn new(vertex_count: usize) -> EdgesComponent {
      EdgesComponent { vertex_count: vertex_count }
    }
  }

  pub struct ShapeComponent {
    pub shape: ShapeType
  }

  impl ShapeComponent {
    pub fn new(shape: ShapeType) -> ShapeComponent {
      ShapeComponent { shape: shape }
    }
  }

  pub enum ShapeType {
    NStar,
    NGon2D,
    NGon3D,
  }
}

pub trait GeometryInterface {
  fn find_edge_component_for(&self, entity_id: usize)
    -> Option<&mut EdgesComponent>;

  fn find_shape_component_for(&self, entity_id: usize);
    -> Option<&mut ShapeComponent>;
  }
}
