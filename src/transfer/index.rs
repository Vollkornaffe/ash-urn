use crate::UrnError;
use crate::Base;

use crate::mesh::Indices;
use crate::{DeviceBuffer, DeviceBufferSettings};

use super::copy_buffer_to_buffer;
use super::create_staging_buffer;

use ash::version::DeviceV1_0;

pub fn create_index_buffer(
    base: &Base,
    indices: &[Indices],
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceBuffer, UrnError> {

    let size = (indices.len() * std::mem::size_of::<Indices>()) as ash::vk::DeviceSize;

    let staging = create_staging_buffer(base, size, format!("{}Staging", name.clone()))?;

    let data_ptr = unsafe {
        base.logical_device.0.map_memory(
            staging.memory.0,
            0,
            size,
            ash::vk::MemoryMapFlags::default(),
        )?
    } as *mut Indices;

    unsafe {
        data_ptr.copy_from_nonoverlapping(indices.as_ptr(), indices.len());
        base.logical_device.0.unmap_memory(staging.memory.0);
    }

    let index = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::INDEX_BUFFER
                 | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            name,
        },
    )?;

    copy_buffer_to_buffer(base, queue, pool, staging.buffer.0, index.buffer.0, size)?;
        
    staging.destroy(base);

    Ok(index)
}
