use crate::AppError;

use ash_urn::Mesh;
use ash_urn::Vertex;

pub fn load_mesh(filename: &'static str) -> Result<Mesh, AppError> {
    let (gltf, buffers, _) = gltf::import(filename)?;
    let mesh = gltf.meshes().nth(0).unwrap();
    let primitive = mesh.primitives().nth(0).unwrap();
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    Ok(Mesh {
        vertices: reader
            .read_positions()
            .unwrap()
            .map(|p| Vertex {
                pos: p.into(),
                col: [1.0, 0.0, 0.0, 1.0].into(),
            })
            .collect(),
        indices: reader.read_indices().unwrap().into_u32().collect(),
    })
}
