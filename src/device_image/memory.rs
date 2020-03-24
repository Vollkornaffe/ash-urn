use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Memory(pub ash::vk::DeviceMemory);

pub struct MemorySettings {
    pub properties: ash::vk::MemoryPropertyFlags,
    pub image: ash::vk::Image,
    pub name: String,
}

impl Memory {
    pub fn alloc(
        base: &Base,
        settings: &MemorySettings,
    ) -> Result<Self, UrnError> {
        let memory_requirements =
            unsafe { base.logical_device.0.get_image_memory_requirements(settings.image) };

        let alloc_info = ash::vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(base.find_memory_type_index(
                ash::vk::MemoryPropertyFlags::from_raw(memory_requirements.memory_type_bits),
                settings.properties,
            )?);
        let image_memory = unsafe { base.logical_device.0.allocate_memory(&alloc_info, None)? };
        base.name_object(image_memory, settings.name.clone())?;

        unsafe {
            base.logical_device.0
                .bind_image_memory(settings.image, image_memory, 0)?
        };

        Ok(Self(image_memory))

    }
}
