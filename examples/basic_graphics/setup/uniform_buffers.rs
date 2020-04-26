use crate::AppError;
use crate::UBO;

use ash_urn::Base;
use ash_urn::{DeviceBuffer, DeviceBufferSettings};

pub fn setup(base: &Base, n_buffer: u32) -> Result<Vec<DeviceBuffer>, AppError> {
    let mut uniform_buffers = Vec::new();
    for i in 0..n_buffer {
        uniform_buffers.push(DeviceBuffer::new(
            &base,
            &DeviceBufferSettings {
                size: std::mem::size_of::<UBO>() as ash::vk::DeviceSize,
                usage: ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                properties: ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                    | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
                shared: false,
                name: format!("UniformBuffer_{}", i),
            },
        )?);
    }

    Ok(uniform_buffers)
}
