use crate::mesh::Vertex;

impl Vertex {

    pub fn get_binding_description() -> [ash::vk::VertexInputBindingDescription; 1] {
        [
            ash::vk::VertexInputBindingDescription::builder()
                .binding(0)
                .stride(std::mem::size_of::<Self>() as u32)
                .input_rate(ash::vk::VertexInputRate::VERTEX)
                .build()
        ]
    }

    pub fn get_attribute_description() -> [ash::vk::VertexInputAttributeDescription; 4] {
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
