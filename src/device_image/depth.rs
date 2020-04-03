use crate::Base;
use crate::UrnError;

use super::{DeviceImage, DeviceImageSettings};

pub fn create_depth_device_image(
    base: &Base,
    swapchain_extent: ash::vk::Extent2D,
) -> Result<DeviceImage, UrnError> {
    let format = base.find_supported_format(
        vec![
            ash::vk::Format::D32_SFLOAT,
            ash::vk::Format::D32_SFLOAT_S8_UINT,
            ash::vk::Format::D24_UNORM_S8_UINT,
        ],
        ash::vk::ImageTiling::OPTIMAL,
        ash::vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )?;

    DeviceImage::new(
        base,
        &DeviceImageSettings {
            width: swapchain_extent.width,
            height: swapchain_extent.height,
            format,
            tiling: ash::vk::ImageTiling::OPTIMAL,
            usage: ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            properties: ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
            aspect_flags: ash::vk::ImageAspectFlags::DEPTH,
            name: "Depth".to_string(),
        },
    )
}
