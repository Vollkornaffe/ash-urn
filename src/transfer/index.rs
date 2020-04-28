use crate::Base;
use crate::UrnError;

use crate::{DeviceBuffer, DeviceBufferSettings};

use super::copy_buffer_to_buffer;
use super::create_staging_device_buffer;

use ash::version::DeviceV1_0;

pub fn create_index_device_buffer(
    base: &Base,
    indices: &[u32],
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceBuffer, UrnError> {
    let size = (indices.len() * std::mem::size_of::<u32>()) as ash::vk::DeviceSize;

    let staging = create_staging_device_buffer(base, size, format!("{}Staging", name.clone()))?;

    staging.write_slice(base, indices)?;

    let index = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::INDEX_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            map: false,
            shared: false,
            name,
        },
    )?;

    copy_buffer_to_buffer(base, queue, pool, staging.buffer.0, index.buffer.0, size)?;

    staging.destroy(base);

    Ok(index)
}
