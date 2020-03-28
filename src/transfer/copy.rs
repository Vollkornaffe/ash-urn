use crate::UrnError;
use crate::Base;

use crate::command::single_time;

use ash::version::DeviceV1_0;

pub fn copy_buffer_to_image(
    base: &Base,
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    buffer: ash::vk::Buffer,
    image: ash::vk::Image,
    width: u32,
    height: u32,
) -> Result<(), UrnError> {

    let command_buffer = single_time::begin(base, pool, "CopyBufferToImage".to_string())?;

    let region = ash::vk::BufferImageCopy::builder()
        .buffer_offset(0)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(
            ash::vk::ImageSubresourceLayers::builder()
                .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
                .build(),
        )
        .image_offset(ash::vk::Offset3D { x: 0, y: 0, z: 0 })
        .image_extent(ash::vk::Extent3D {
            width,
            height,
            depth: 1,
        });

    let regions = [region.build()];
    unsafe {
        base.logical_device.0.cmd_copy_buffer_to_image(
            command_buffer,
            buffer,
            image,
            ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &regions,
        )
    };

    single_time::end(base, queue, pool, command_buffer)
}

pub fn copy_buffer_to_buffer(
    base: &Base,
    queue: ash::vk::Queue,
    pool: ash::vk::CommandPool,
    src_buffer: ash::vk::Buffer,
    dst_buffer: ash::vk::Buffer,
    size: ash::vk::DeviceSize,
) -> Result<(), UrnError> {

    let command_buffer = single_time::begin(base, pool, "CopyBufferToBuffer".to_string())?;

    let copy_region = ash::vk::BufferCopy::builder()
        .src_offset(0)
        .dst_offset(0)
        .size(size);
    let regions = [copy_region.build()];
    unsafe {
        base.logical_device.0
            .cmd_copy_buffer(command_buffer, src_buffer, dst_buffer, &regions)
    };

    single_time::end(base, queue, pool, command_buffer)
}
