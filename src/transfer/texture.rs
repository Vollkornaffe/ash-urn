use crate::Base;
use crate::UrnError;

use crate::{DeviceImage, DeviceImageSettings};
use super::create_staging_device_buffer;
use super::copy_buffer_to_image;


use ash::version::DeviceV1_0;

pub fn create_texture_device_image(
   base: &Base,
   filename: String,
   queue: ash::vk::Queue,
   pool: ash::vk::CommandPool,
   name: String,
) -> Result<DeviceImage, UrnError> {

    let buffer = ::image::open(filename)?.to_rgba();
    let size = (buffer.width() * buffer.height() * 4) as ash::vk::DeviceSize;

    let staging = create_staging_device_buffer(base, size, format!("{}Staging", name.clone()))?;

    let data_ptr = unsafe {
        base.logical_device.0.map_memory(
            staging.memory.0,
            0,
            size,
            ash::vk::MemoryMapFlags::default(),
        )?
    } as *mut u8;

    unsafe {
        data_ptr.copy_from_nonoverlapping(buffer.as_ptr(), buffer.len());
        base.logical_device.0.unmap_memory(staging.memory.0)
    };

    let texture = DeviceImage::new(
        base,
        &DeviceImageSettings {
            width: buffer.width(),
            height: buffer.height(),
            format: ash::vk::Format::R8G8B8A8_UNORM,
            tiling: ash::vk::ImageTiling::OPTIMAL,
            usage: ash::vk::ImageUsageFlags::TRANSFER_DST
                | ash::vk::ImageUsageFlags::SAMPLED,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            aspect_flags: ash::vk::ImageAspectFlags::COLOR,
            name,
        },
    )?;

    copy_buffer_to_image(
        base,
        queue,
        pool,
        staging.buffer.0,
        texture.image.0,
        buffer.width(),
        buffer.height(),
    )?;

    staging.destroy(base);

    Ok(texture)

}
