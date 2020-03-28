use crate::UrnError;
use crate::Base;

use crate::{DeviceBuffer, DeviceBufferSettings};

pub fn create_staging_buffer(
    base: &Base,
    size: ash::vk::DeviceSize,
    name: String,
) -> Result<DeviceBuffer, UrnError> {
    
    DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::TRANSFER_SRC,
            properties: ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                      | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
            name,
        },
    )

}
