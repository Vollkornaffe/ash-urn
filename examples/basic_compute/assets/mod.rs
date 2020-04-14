use crate::AppError;

use ash_urn::Mesh;
use ash_urn::Vertex;

use itertools::izip;

pub fn load_mesh(filename: &'static str) -> Result<Mesh, AppError> {
    let gltf = gltf::Gltf::open(filename)?;
    for mesh in gltf.meshes() {
        println!("Mesh #{}", mesh.index());
        for primitive in mesh.primitives() {
            println!("- Primitive #{}", primitive.index());
            for (semantic, _) in primitive.attributes() {
                println!("-- {:?}", semantic);
            }
        }
    }

    let (gltf, buffers, _) = gltf::import(filename)?;
    let mesh = gltf.meshes().nth(0).unwrap();
    let primitive = mesh.primitives().nth(0).unwrap();
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    Ok(Mesh {
        vertices: izip!(
            reader.read_positions().unwrap(),
            reader.read_normals().unwrap(),
            reader.read_colors(0).unwrap().into_rgba_f32(),
            reader.read_tex_coords(0).unwrap().into_f32(),
        )
        .map(|(p, n, c, t)| Vertex {
            pos: p.into(),
            nor: n.into(),
            col: c.into(),
            tex: t.into(),
        })
        .collect(),
        indices: reader.read_indices().unwrap().into_u32().collect(),
    })
}
