use crate::Base;
use crate::UrnError;
use crate::base::queue_families::{COMBINED, DEDICATED_TRANSFER};

use ash::version::DeviceV1_0;

pub struct Buffer(pub ash::vk::Buffer);

pub struct BufferSettings {
    pub size: ash::vk::DeviceSize,
    pub usage: ash::vk::BufferUsageFlags,
    pub shared: bool,
    pub name: String,
}

impl Buffer {

    pub fn new(base: &Base, settings: &BufferSettings) -> Result<Self, UrnError> {

        let queue_family_indices = [
            base.queue_map.get(&COMBINED).unwrap().idx,
            base.queue_map.get(&DEDICATED_TRANSFER).unwrap().idx,
        ];

        let buffer_info = if settings.shared {
            ash::vk::BufferCreateInfo::builder()
                .size(settings.size)
                .usage(settings.usage)
                .sharing_mode(ash::vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_family_indices)
        } else {
             ash::vk::BufferCreateInfo::builder()
                .size(settings.size)
                .usage(settings.usage)
                .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
        };

        let buffer = unsafe { base.logical_device.0.create_buffer(&buffer_info, None)? };
        base.name_object(buffer, settings.name.clone())?;

        Ok(Self(buffer))
    }

}
