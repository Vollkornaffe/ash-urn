use crate::AppError;
use crate::ComputeUBO;
use crate::GraphicsUBO;

use ash_urn::Base;
use ash_urn::{DeviceBuffer, DeviceBufferSettings};

pub fn setup_graphics(base: &Base, n_buffer: u32) -> Result<Vec<DeviceBuffer>, AppError> {
    let mut uniform_buffers = Vec::new();
    for i in 0..n_buffer {
        uniform_buffers.push(DeviceBuffer::new(
            &base,
            &DeviceBufferSettings {
                size: std::mem::size_of::<GraphicsUBO>() as ash::vk::DeviceSize,
                usage: ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                properties: ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                    | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
                name: format!("UniformBuffer_{}", i),
            },
        )?);
    }

    Ok(uniform_buffers)
}

pub fn setup_compute(base: &Base) -> Result<DeviceBuffer, AppError> {
    let uniform_buffer = DeviceBuffer::new(
        &base,
        &DeviceBufferSettings {
            size: std::mem::size_of::<ComputeUBO>() as ash::vk::DeviceSize,
            usage: ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
            properties: ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
            name: "ComputeUBO".to_string(),
        },
    )?;

    Ok(uniform_buffer)
}
