use std::any::Any;

use crate::{Component, Vertex};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
}

impl Component for Mesh {
  fn name(&self) -> &'static str {
    "Mesh"
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn inspect(&mut self, ui: &mut egui::Ui) {
    egui::Grid::new("mesh").num_columns(2).spacing([8.0, 4.0]).show(ui, |ui| {
      ui.label("Vertices");
      ui.label(self.vertices.len().to_string());
      ui.end_row();
      ui.label("Indices");
      ui.label(self.indices.len().to_string());
      ui.end_row();
      ui.label("Triangles");
      ui.label((self.indices.len() / 3).to_string());
      ui.end_row();
    });
  }
}

impl Mesh {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
    Self { vertices, indices }
  }

  pub fn cube() -> Self {
    Self::new(CUBE_VERTICES.to_vec(), CUBE_INDICES.to_vec())
  }
}

// 24 vertices (4 per face) so each vertex carries a single unambiguous face normal.
// Winding is CCW when viewed from outside (front_face = Ccw, cull_mode = Back).
#[rustfmt::skip]
const CUBE_VERTICES: &[Vertex] = &[
  // Front  (+Z)
  Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
  // Back   (-Z)
  Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
  // Top    (+Y)
  Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0] },
  // Bottom (-Y)
  Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0] },
  // Right  (+X)
  Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 1.0,  0.0,  0.0] },
  // Left   (-X)
  Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0] },
];

#[rustfmt::skip]
const CUBE_INDICES: &[u16] = &[
   0,  1,  2,   0,  2,  3,  // Front
   4,  5,  6,   4,  6,  7,  // Back
   8,  9, 10,   8, 10, 11,  // Top
  12, 13, 14,  12, 14, 15,  // Bottom
  16, 17, 18,  16, 18, 19,  // Right
  20, 21, 22,  20, 22, 23,  // Left
];
