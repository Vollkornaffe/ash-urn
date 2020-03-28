use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub fn begin(
    base: &Base,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<ash::vk::CommandBuffer, UrnError> {

    let alloc_info = ash::vk::CommandBufferAllocateInfo::builder()
        .level(ash::vk::CommandBufferLevel::PRIMARY)
        .command_pool(pool)
        .command_buffer_count(1);

    let command_buffer = unsafe { base.logical_device.0.allocate_command_buffers(&alloc_info)? }[0];
    base.name_object(command_buffer, name)?;

    let begin_info = ash::vk::CommandBufferBeginInfo::builder()
        .flags(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    unsafe {
        base.logical_device.0
            .begin_command_buffer(command_buffer, &begin_info)?
    };

    Ok(command_buffer)
}

pub fn end(
    base: &Base,
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    command_buffer: ash::vk::CommandBuffer,
) -> Result<(), UrnError> {
    unsafe { base.logical_device.0.end_command_buffer(command_buffer)? };

    let command_buffers = [command_buffer];
    let submit_info = ash::vk::SubmitInfo::builder().command_buffers(&command_buffers);
    let submit_infos = [submit_info.build()];
    unsafe {
        base.logical_device.0
            .queue_submit(queue, &submit_infos, ash::vk::Fence::default())?
    };

    unsafe { base.logical_device.0.queue_wait_idle(queue)? };
    unsafe {
        base.logical_device.0
            .free_command_buffers(pool, &command_buffers);
    }

    Ok(())
}


