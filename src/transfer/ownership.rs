use crate::UrnError;
use crate::Base;
use crate::DeviceBuffer;
use crate::DeviceImage;
use crate::Command;
use crate::command::single_time;

use ash::version::DeviceV1_0;

pub fn transfer_release(
    base: &Base,
    memory_barriers: &[ash::vk::MemoryBarrier],
    buffer_memory_barriers: &[ash::vk::BufferMemoryBarrier],
    image_memory_barriers: &[ash::vk::ImageMemoryBarrier],
    transfer_command: &Command,
) -> Result<(), UrnError> {

    let transfer_command_buffer = single_time::begin(
        base,
        transfer_command.pool.0,
        "OwnerShipRelease".to_string(),
    )?;

    unsafe {
        base.logical_device.0.cmd_pipeline_barrier(
            transfer_command_buffer,
            ash::vk::PipelineStageFlags::TRANSFER,
            ash::vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            ash::vk::DependencyFlags::default(), // TODO
            memory_barriers,
            buffer_memory_barriers,
            image_memory_barriers,
        )
    }

    single_time::end(
        base,
        transfer_command.queue.0,
        transfer_command.pool.0,
        transfer_command_buffer,
    )

}

pub fn combined_acquire(
    base: &Base,
    memory_barriers: &[ash::vk::MemoryBarrier],
    buffer_memory_barriers: &[ash::vk::BufferMemoryBarrier],
    image_memory_barriers: &[ash::vk::ImageMemoryBarrier],
    combined_command: &Command,
) -> Result<(), UrnError> {

    let combined_command_buffer = single_time::begin(
        base,
        combined_command.pool.0,
        "OwnerShipAquire".to_string(),
    )?;

    unsafe {
        base.logical_device.0.cmd_pipeline_barrier(
            combined_command_buffer,
            ash::vk::PipelineStageFlags::TOP_OF_PIPE,
            ash::vk::PipelineStageFlags::VERTEX_INPUT
                | ash::vk::PipelineStageFlags::COMPUTE_SHADER
                | ash::vk::PipelineStageFlags::FRAGMENT_SHADER,
            ash::vk::DependencyFlags::default(), // TODO
            &memory_barriers,
            buffer_memory_barriers,
            image_memory_barriers,
        )
    }

    single_time::end(
        base,
        combined_command.queue.0,
        combined_command.pool.0,
        combined_command_buffer,
    )

}

/// Assumes images are readonly in shader
pub fn transfer_to_combined(
    base: &Base,
    device_buffers: &[&DeviceBuffer],
    device_images: &[&DeviceImage],
    transfer_command: &Command,
    combined_command: &Command,
) -> Result<(), UrnError> {

    let memory_barriers = Vec::new();
    let mut buffer_memory_barriers = Vec::new();
    let mut image_memory_barriers = Vec::new();

    for device_buffer in device_buffers {
        buffer_memory_barriers.push(
            ash::vk::BufferMemoryBarrier::builder()
                .src_queue_family_index(transfer_command.family_idx)
                .dst_queue_family_index(combined_command.family_idx)
                .buffer(device_buffer.buffer.0)
                .offset(0)
                .size(ash::vk::WHOLE_SIZE)
                .build(),
        );
    }

    for device_image in device_images {
        image_memory_barriers.push(
            ash::vk::ImageMemoryBarrier::builder()
                .old_layout(ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .src_queue_family_index(transfer_command.family_idx)
                .dst_queue_family_index(combined_command.family_idx)
                .image(device_image.image.0)
                .subresource_range(
                    ash::vk::ImageSubresourceRange::builder()
                        .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                )
                .build(),
        );
    }


    // RELEASE OWNER SHIP +++++++++++++++++++++++++++++++++++++++++++++++++++++
    for buffer_memory_barrier in &mut buffer_memory_barriers {
        buffer_memory_barrier.src_access_mask = ash::vk::AccessFlags::TRANSFER_WRITE;
        buffer_memory_barrier.dst_access_mask = ash::vk::AccessFlags::default();
    }
    for image_memory_barrier in &mut image_memory_barriers {
        image_memory_barrier.src_access_mask = ash::vk::AccessFlags::TRANSFER_WRITE;
        image_memory_barrier.dst_access_mask = ash::vk::AccessFlags::default();
    }
    transfer_release(
        base,
        memory_barriers.as_slice(),
        buffer_memory_barriers.as_slice(),
        image_memory_barriers.as_slice(),
        transfer_command,
    )?;

    // AQUIRE OWNER SHIP ++++++++++++++++++++++++++++++++++++++++++++++++++++++
    for buffer_memory_barrier in &mut buffer_memory_barriers {
        buffer_memory_barrier.src_access_mask = ash::vk::AccessFlags::default();
        buffer_memory_barrier.dst_access_mask = ash::vk::AccessFlags::VERTEX_ATTRIBUTE_READ
            | ash::vk::AccessFlags::SHADER_READ
            | ash::vk::AccessFlags::SHADER_WRITE;
    }
    for image_memory_barrier in &mut image_memory_barriers {
        image_memory_barrier.src_access_mask = ash::vk::AccessFlags::default();
        image_memory_barrier.dst_access_mask = ash::vk::AccessFlags::SHADER_READ;
    }
    combined_acquire(
        base,
        memory_barriers.as_slice(),
        buffer_memory_barriers.as_slice(),
        image_memory_barriers.as_slice(),
        combined_command,
    )?;

    Ok(())
}
