use crate::Base;
use crate::UrnError;

use crate::{DeviceBuffer, DeviceBufferSettings};

use super::copy_buffer_to_buffer;
use super::create_staging_device_buffer;

use ash::version::DeviceV1_0;

pub fn create_storage_device_buffer<T>(
    base: &Base,
    data: &[T],
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceBuffer, UrnError> {
    let size = (data.len() * std::mem::size_of::<T>()) as ash::vk::DeviceSize;

    let staging = create_staging_device_buffer(base, size, format!("{}Staging", name.clone()))?;

    let data_ptr = unsafe {
        base.logical_device.0.map_memory(
            staging.memory.0,
            0,
            size,
            ash::vk::MemoryMapFlags::default(),
        )?
    } as *mut T;

    unsafe {
        data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
        base.logical_device.0.unmap_memory(staging.memory.0);
    }

    let storage = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::STORAGE_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_SRC
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            shared: false,
            name,
        },
    )?;

    copy_buffer_to_buffer(base, queue, pool, staging.buffer.0, storage.buffer.0, size)?;

    staging.destroy(base);

    Ok(storage)
}

pub fn create_storage_device_buffer_uninitialized<T>(
    base: &Base,
    len: usize,
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceBuffer, UrnError> {
    let size = (len * std::mem::size_of::<T>()) as ash::vk::DeviceSize;

    let storage = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::STORAGE_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_SRC
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            shared: false,
            name,
        },
    )?;

    Ok(storage)
}
