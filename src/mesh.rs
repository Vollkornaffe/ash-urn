use crate::memory_alignment::Align16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pos(pub [f32; 3]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Col(pub [f32; 4]);

#[repr(C, align(64))]
pub struct Vertex {
    pub pos: Align16<Pos>,
    pub col: Align16<Col>,
}

#[repr(C)]
pub struct Indices(pub [u32; 3]);

#[repr(C)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Indices>,
}

impl Vertex {
    pub fn get_binding_description() -> [ash::vk::VertexInputBindingDescription; 1] {
        [ash::vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<Self>() as u32)
            .input_rate(ash::vk::VertexInputRate::VERTEX)
            .build()]
    }

    pub fn get_attribute_description() -> [ash::vk::VertexInputAttributeDescription; 2] {
        [
            ash::vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(memoffset::offset_of!(Self, pos) as u32)
                .build(),
            ash::vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(ash::vk::Format::R32G32B32A32_SFLOAT)
                .offset(memoffset::offset_of!(Self, col) as u32)
                .build(),
        ]
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn add_quad(
        &mut self,
        c0: Pos,
        c1: Pos,
        c2: Pos,
        c3: Pos,
        col: Col,
    ) {
        let offset = self.vertices.len() as u32;
        self.vertices.push(Vertex {pos: c0.into(), col: col.into()});
        self.vertices.push(Vertex {pos: c1.into(), col: col.into()});
        self.vertices.push(Vertex {pos: c2.into(), col: col.into()});
        self.vertices.push(Vertex {pos: c3.into(), col: col.into()});
        self.indices.push(Indices([offset + 1, offset + 0, offset + 2]));
        self.indices.push(Indices([offset + 3, offset + 2, offset + 0]));
    }
}
