use cgmath::prelude::*;

use ash_urn::memory_alignment::Align16;
use ash_urn::UrnMesh;
use ash_urn::UrnVertex;

#[repr(C, align(32))]
pub struct Particle {
    pub pos: Align16<cgmath::Vector3<f32>>,
    pub vel: Align16<cgmath::Vector3<f32>>,
}

pub struct Particles(pub Vec<Particle>);

impl Particles {
    pub fn new(res: usize) -> Self {
        let n_particles = res * res * res;

        let mut particles = Vec::new();

        particles.reserve(n_particles);

        for i in 0..res {
            for j in 0..res {
                for k in 0..res {

                    let pos = cgmath::Vector3::<f32>::new(
                        (0.5 + i as f32) / res as f32 - 0.5,
                        (0.5 + j as f32) / res as f32 - 0.5,
                        (0.5 + k as f32) / res as f32 - 0.5,
                    );

                    particles.push(Particle {
                        pos: pos.into(),
                        vel: cgmath::Vector3::<f32>::new(-15.0 * pos.y, 5.0 * pos.x, -pos.z).into(),
                    });
                }
            }
        }

        Self(particles)
    }

    pub fn as_mesh(&self, reference: &UrnMesh, scale: f32) -> UrnMesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        vertices.reserve(reference.vertices.len() * self.0.len());
        indices.reserve(reference.indices.len() * self.0.len());

        for p in &self.0 {
            let idx_offset = vertices.len() as u32;
            let offset = p.pos.0;
            for v in &reference.vertices {
                vertices.push(UrnVertex {
                    pos: [
                        v.pos.0[0] * scale + offset[0],
                        v.pos.0[1] * scale + offset[1],
                        v.pos.0[2] * scale + offset[2],
                    ]
                    .into(),
                    nor: v.nor,
                    col: v.col,
                    tex: [offset[0], offset[1]].into(),
                });
            }
            for i in &reference.indices {
                indices.push(i + idx_offset);
            }
        }

        UrnMesh { vertices, indices }
    }
}
