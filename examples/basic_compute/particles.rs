use cgmath::prelude::*;

use ash_urn::Mesh;
use ash_urn::Vertex;

#[repr(C)]
pub struct Particles {
    pub n_particles: usize,
    pub positions: Vec<cgmath::Vector3<f32>>,
    pub velocities: Vec<cgmath::Vector3<f32>>,
}

impl Particles {
    pub fn new(res: usize) -> Self {

        let n_particles = res * res * res;

        let mut positions = Vec::new();
        let mut velocities = Vec::new();

        positions.reserve(n_particles);
        velocities.resize(n_particles, cgmath::Vector3::<f32>::zero());

        for i in 0..res {
            for j in 0..res {
                for k in 0..res {
                    positions.push(cgmath::Vector3::<f32>::new(
                        (0.5 + i as f32) / res as f32 - 0.5,
                        (0.5 + j as f32) / res as f32 - 0.5,
                        (0.5 + k as f32) / res as f32 - 0.5,
                    ));
                }
            }
        }

        Self {
            n_particles,
            positions,
            velocities,
        }
    }

    pub fn as_mesh(&self, reference: &Mesh, scale: f32) -> Mesh {

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        vertices.reserve(reference.vertices.len() * self.n_particles);
        indices.reserve(reference.indices.len() * self.n_particles);

        for offset in &self.positions {
            let idx_offset = vertices.len() as u32;
            for v in &reference.vertices {
                vertices.push(Vertex {
                    pos: [
                        v.pos.0[0] * scale + offset[0],
                        v.pos.0[1] * scale + offset[1],
                        v.pos.0[2] * scale + offset[2],
                    ].into(),
                    nor: v.nor,
                    col: v.col,
                    tex: v.tex,
                });
                for i in &reference.indices {
                    indices.push(i + idx_offset);
                }
            }
        } 

        Mesh {
            vertices,
            indices,
        }
    }
}
