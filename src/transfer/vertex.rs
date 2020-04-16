use crate::Base;
use crate::UrnError;

use crate::Vertex;
use crate::{DeviceBuffer, DeviceBufferSettings};

use super::copy_buffer_to_buffer;
use super::create_staging_device_buffer;

use ash::version::DeviceV1_0;

pub fn create_vertex_device_buffer<V: Vertex>(
    base: &Base,
    vertices: &[V],
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceBuffer, UrnError> {
    let size = (vertices.len() * std::mem::size_of::<V>()) as ash::vk::DeviceSize;

    let staging = create_staging_device_buffer(base, size, format!("{}Staging", name.clone()))?;

    let data_ptr = unsafe {
        base.logical_device.0.map_memory(
            staging.memory.0,
            0,
            size,
            ash::vk::MemoryMapFlags::default(),
        )?
    } as *mut V;

    unsafe {
        data_ptr.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len());
        base.logical_device.0.unmap_memory(staging.memory.0);
    }

    let vertex = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::VERTEX_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
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

    let data_ptr = unsafe {
        base.logical_device.0.map_memory(
            staging.memory.0,
            0,
            size,
            ash::vk::MemoryMapFlags::default(),
        )?
    } as *mut V;

    unsafe {
        data_ptr.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len());
        base.logical_device.0.unmap_memory(staging.memory.0);
    }

    let vertex = DeviceBuffer::new(
        base,
        &DeviceBufferSettings {
            size,
            usage: ash::vk::BufferUsageFlags::VERTEX_BUFFER
                | ash::vk::BufferUsageFlags::STORAGE_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            name,
        },
    )?;

    copy_buffer_to_buffer(base, queue, pool, staging.buffer.0, vertex.buffer.0, size)?;

    staging.destroy(base);

    Ok(vertex)
}
