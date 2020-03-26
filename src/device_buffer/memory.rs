use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Memory(ash::vk::DeviceMemory);

pub struct MemorySettings {
    properties: ash::vk::MemoryPropertyFlags,
    buffer: ash::vk::Buffer,
    name: String,
}

impl Memory {
    pub fn alloc(base: &Base, settings: MemorySettings) -> Result<Self, UrnError> {
        let memory_requirements = unsafe {
            base.logical_device
                .0
                .get_buffer_memory_requirements(settings.buffer)
        };

        let memory_type_index = base.find_memory_type_index(
            ash::vk::MemoryPropertyFlags::from_raw(memory_requirements.memory_type_bits),
            settings.properties,
        )?;

        let alloc_info = ash::vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let buffer_memory = unsafe { base.logical_device.0.allocate_memory(&alloc_info, None)? };
        base.name_object(buffer_memory, settings.name.clone())?;

        unsafe {
            base.logical_device
                .0
                .bind_buffer_memory(settings.buffer, buffer_memory, 0)?
        };

        Ok(Self(buffer_memory))
    }
}
