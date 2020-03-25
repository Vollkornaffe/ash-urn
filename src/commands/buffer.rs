use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Buffer(pub ash::vk::CommandBuffer);

impl Buffer {
    pub fn alloc(
        base: &Base,
        pool: ash::vk::CommandPool,
        name: String,
    ) -> Result<Self, UrnError> {
        let alloc_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_pool(pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let buffers =
            unsafe { base.logical_device.0.allocate_command_buffers(&alloc_info)? };
        let buffer = buffers.first().unwrap();
        base.name_object(*buffer, name)?;

        Ok(Self(*buffer))
    }

    pub fn alloc_vec(
        base: &Base,
        pool: ash::vk::CommandPool,
        n_buffer: u32,
        name: String,
    ) -> Result<Vec<Self>, UrnError> {
        let alloc_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_pool(pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(n_buffer);

        let buffers =
            unsafe { base.logical_device.0.allocate_command_buffers(&alloc_info)? };
        for i in 0..n_buffer {
            base.name_object(
                buffers[i as usize],
                format!("{}_{}", name, i),
            )?;
        }

        Ok(buffers.into_iter().map(Self).collect())
    }
}
