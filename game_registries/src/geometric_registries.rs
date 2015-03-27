mod geometric_registries {
  pub struct DynamicVertexComponent {
    vertex_count: usize
  }

  impl DynamicVertexComponent {
    pub fn new(vertex_count: usize) -> DynamicVertexComponent {
      DynamicVertexComponent{ vertex_count: vertex_count }
    }

    // TODO: Move this into game_geometry
    pub fn inc_vertices(&mut self) {
      self.vertex_count += 1;
    }

    pub fn dec_vertices(&mut self) {
      if self.edge_count != 0usize {
        self.edge_count -= 1;
      }
    }

    pub fn vertex_count(&self) -> usize {
      self.vertex_count
    }
  }

  pub struct ShapeComponent {
    shape: ShapeType
  }

  pub enum ShapeType {
    NStar,
    NGon2D,
    NGon3D,
  }

  pub struct MeshComponent {
    something: i32
  }
}
