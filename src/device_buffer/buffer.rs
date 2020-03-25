use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Buffer(ash::vk::Buffer);

pub struct BufferSettings {
    size: ash::vk::DeviceSize,
    usage: ash::vk::BufferUsageFlags,
    name: String,
}

impl Buffer {

    // only exclusive buffers are supported atm
    pub fn new(
        base: &Base,
        settings: &BufferSettings,
    ) -> Result<Self, UrnError> {
        
        let buffer_info = ash::vk::BufferCreateInfo::builder()
                .size(settings.size)
                .usage(settings.usage)
                .sharing_mode(ash::vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { base.logical_device.0.create_buffer(&buffer_info, None)? };
        base.name_object(buffer, settings.name.clone())?;

        Ok(Self(buffer))
    }
}
