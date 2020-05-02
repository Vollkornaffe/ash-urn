use crate::Base;
use crate::UrnError;

use super::copy_buffer_to_image;
use super::create_staging_device_buffer;
use crate::command::image_layout;
use crate::command::image_layout::TransitionImageLayoutSettings;
use crate::{DeviceImage, DeviceImageSettings};

pub fn create_texture_device_image(
    base: &Base,
    filename: String,
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    name: String,
) -> Result<DeviceImage, UrnError> {
    let buffer = ::image::open(filename)?.to_rgba();
    let width = buffer.width();
    let height = buffer.height();

    let size = (buffer.width() * buffer.height() * 4) as ash::vk::DeviceSize;

    let staging = create_staging_device_buffer(base, size, format!("{}Staging", name.clone()))?;

    staging.write_slice(base, buffer.into_raw().as_slice())?;

    let texture = DeviceImage::new(
        base,
        &DeviceImageSettings {
            width,
            height,
            format: ash::vk::Format::R8G8B8A8_UNORM,
            tiling: ash::vk::ImageTiling::OPTIMAL,
            usage: ash::vk::ImageUsageFlags::TRANSFER_DST | ash::vk::ImageUsageFlags::SAMPLED,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            aspect_flags: ash::vk::ImageAspectFlags::COLOR,
            name,
        },
    )?;

    image_layout::transition(
        base,
        &TransitionImageLayoutSettings {
            queue,
            pool,
            image: texture.image.0,
            aspect_mask: ash::vk::ImageAspectFlags::COLOR,
            old_layout: ash::vk::ImageLayout::UNDEFINED,
            new_layout: ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            src_access: ash::vk::AccessFlags::default(),
            dst_access: ash::vk::AccessFlags::TRANSFER_WRITE,
            src_stage: ash::vk::PipelineStageFlags::TOP_OF_PIPE,
            dst_stage: ash::vk::PipelineStageFlags::TRANSFER,
        },
    )?;

    copy_buffer_to_image(
        base,
        queue,
        pool,
        staging.buffer.0,
        texture.image.0,
        width,
        height,
    )?;

    staging.destroy(base);

    Ok(texture)
}
