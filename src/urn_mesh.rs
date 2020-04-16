use crate::memory_alignment::Align16;

pub trait Vertex {
    fn get_binding_description() -> Vec<ash::vk::VertexInputBindingDescription>;
    fn get_attribute_description() -> Vec<ash::vk::VertexInputAttributeDescription>;
}

#[repr(C, align(64))]
#[derive(Debug)]
pub struct UrnVertex {
    pub pos: Align16<[f32; 3]>,
    pub nor: Align16<[f32; 3]>,
    pub col: Align16<[f32; 4]>,
    pub tex: Align16<[f32; 2]>,
}

impl Default for UrnVertex {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0].into(),
            nor: [0.0, 0.0, 0.0].into(),
            col: [0.0, 0.0, 0.0, 0.0].into(),
            tex: [0.0, 0.0].into(),
        }
    }
}

#[repr(C)]
pub struct UrnMesh {
    pub vertices: Vec<UrnVertex>,
    pub indices: Vec<u32>,
}

impl Vertex for UrnVertex {
    fn get_binding_description() -> Vec<ash::vk::VertexInputBindingDescription> {
        vec![ash::vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<Self>() as u32)
            .input_rate(ash::vk::VertexInputRate::VERTEX)
            .build()]
    }

    fn get_attribute_description() -> Vec<ash::vk::VertexInputAttributeDescription> {
        vec![
            ash::vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(memoffset::offset_of!(Self, pos) as u32)
                .build(),
            ash::vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(memoffset::offset_of!(Self, nor) as u32)
                .build(),
            ash::vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(2)
                .format(ash::vk::Format::R32G32B32A32_SFLOAT)
                .offset(memoffset::offset_of!(Self, col) as u32)
                .build(),
            ash::vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(3)
                .format(ash::vk::Format::R32G32_SFLOAT)
                .offset(memoffset::offset_of!(Self, tex) as u32)
                .build(),
        ]
    }
}

impl UrnMesh {
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
        self.vertices.push(UrnVertex {
            pos: c0.into(),
            col: col.into(),
            ..Default::default()
        });
        self.vertices.push(UrnVertex {
            pos: c1.into(),
            col: col.into(),
            ..Default::default()
        });
        self.vertices.push(UrnVertex {
            pos: c2.into(),
            col: col.into(),
            ..Default::default()
        });
        self.vertices.push(UrnVertex {
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
