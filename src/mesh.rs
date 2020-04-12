use crate::memory_alignment::Align16;

#[repr(C, align(64))]
pub struct Vertex {
    pub pos: Align16<[f32; 3]>,
    pub nor: Align16<[f32; 3]>,
    pub col: Align16<[f32; 4]>,
    pub tex: Align16<[f32; 2]>,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: [0.0,0.0,0.0].into(),
            nor: [0.0,0.0,0.0].into(),
            col: [0.0,0.0,0.0,0.0].into(),
            tex: [0.0,0.0].into(),
        }
    }
}

#[repr(C)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
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
        mut self,
        c0: [f32; 3],
        c1: [f32; 3],
        c2: [f32; 3],
        c3: [f32; 3],
        col: [f32; 4],
    ) -> Self {
        let offset = self.vertices.len() as u32;
        self.vertices.push(Vertex {
            pos: c0.into(),
            col: col.into(),
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: c1.into(),
            col: col.into(),
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: c2.into(),
            col: col.into(),
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: c3.into(),
            col: col.into(),
            ..Default::default()
        });
        self.indices.push(offset + 1);
        self.indices.push(offset + 0);
        self.indices.push(offset + 2);
        self.indices.push(offset + 3);
        self.indices.push(offset + 2);
        self.indices.push(offset + 0);

        self
    }
}
