use crate::Base;
use crate::UrnError;

use crate::Vertex;
use crate::{DeviceBuffer, DeviceBufferSettings};

use super::copy_buffer_to_buffer;
use super::create_staging_device_buffer;

pub fn create_vertex_device_buffer<V: Vertex>(
    base: &Base,
    vertices: &[V],
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceBuffer, UrnError> {
    let size = (vertices.len() * std::mem::size_of::<V>()) as ash::vk::DeviceSize;

    let staging = create_staging_device_buffer(base, size, format!("{}Staging", name.clone()))?;

    staging.write_slice(base, vertices)?;

    let vertex = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::VERTEX_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            map: false,
            shared: false,
            name,
        },
    )?;

    copy_buffer_to_buffer(base, queue, pool, staging.buffer.0, vertex.buffer.0, size)?;

    staging.destroy(base);

    Ok(vertex)
}

pub fn create_vertex_storage_device_buffer<V: Vertex>(
    base: &Base,
    vertices: &[V],
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceBuffer, UrnError> {
    let size = (vertices.len() * std::mem::size_of::<V>()) as ash::vk::DeviceSize;

    let staging = create_staging_device_buffer(base, size, format!("{}Staging", name.clone()))?;

    staging.write_slice(base, vertices)?;

    let vertex = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::VERTEX_BUFFER
                | ash::vk::BufferUsageFlags::STORAGE_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            map: false,
            shared: false,
            name,
        },
    )?;

    copy_buffer_to_buffer(base, queue, pool, staging.buffer.0, vertex.buffer.0, size)?;

    staging.destroy(base);

    Ok(vertex)
}
