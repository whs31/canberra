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

#[rustfmt::skip]
const CUBE_VERTICES: &[Vertex] = &[
  Vertex { position: [-1.0, -1.0, -1.0] },
  Vertex { position: [ 1.0, -1.0, -1.0] },
  Vertex { position: [ 1.0, -1.0,  1.0] },
  Vertex { position: [-1.0, -1.0,  1.0] },
  Vertex { position: [-1.0,  1.0, -1.0] },
  Vertex { position: [ 1.0,  1.0, -1.0] },
  Vertex { position: [ 1.0,  1.0,  1.0] },
  Vertex { position: [-1.0,  1.0,  1.0] },
];

#[rustfmt::skip]
const CUBE_INDICES: &[u16] = &[
  0, 2, 1,  0, 3, 2,
  4, 5, 6,  4, 6, 7,
  3, 6, 2,  3, 7, 6,
  0, 1, 5,  0, 5, 4,
  0, 7, 3,  0, 4, 7,
  1, 2, 6,  1, 6, 5,
];
