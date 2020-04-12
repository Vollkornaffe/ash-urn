use crate::Base;
use crate::UrnError;

use super::single_time;

use ash::version::DeviceV1_0;

pub struct TransitionImageLayoutSettings {
    pub queue: ash::vk::Queue,
    pub pool: ash::vk::CommandPool,
    pub image: ash::vk::Image,
    pub aspect_mask: ash::vk::ImageAspectFlags,
    pub old_layout: ash::vk::ImageLayout,
    pub new_layout: ash::vk::ImageLayout,
    pub src_access: ash::vk::AccessFlags,
    pub dst_access: ash::vk::AccessFlags,
    pub src_stage: ash::vk::PipelineStageFlags,
    pub dst_stage: ash::vk::PipelineStageFlags,
}

pub fn transition(
    base: &Base,
    settings: &TransitionImageLayoutSettings,
) -> Result<(), UrnError> {

    let command_buffer = single_time::begin(
        base,
        settings.pool,
        "ImageTransition".to_string()
    )?;

    let barrier = ash::vk::ImageMemoryBarrier::builder()
        .old_layout(settings.old_layout)
        .new_layout(settings.new_layout)
        .src_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
        .image(settings.image)
        .subresource_range(
            ash::vk::ImageSubresourceRange::builder()
                .aspect_mask(settings.aspect_mask)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1)
                .build(),
        )
        .src_access_mask(settings.src_access)
        .dst_access_mask(settings.dst_access);

    let memory_barriers = [];
    let buffer_memory_barriers = [];
    let image_memory_barriers = [barrier.build()];
    unsafe {
        base.logical_device.0.cmd_pipeline_barrier(
            command_buffer,
            settings.src_stage,
            settings.dst_stage,
            ash::vk::DependencyFlags::default(), // TODO
            &memory_barriers,
            &buffer_memory_barriers,
            &image_memory_barriers,
        )
    }

    single_time::end(base, settings.queue, settings.pool, command_buffer)?;

    Ok(())
}

