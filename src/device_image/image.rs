use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Image(pub ash::vk::Image);

pub struct ImageSettings {
    pub width: u32,
    pub height: u32,
    pub format: ash::vk::Format,
    pub tiling: ash::vk::ImageTiling,
    pub usage: ash::vk::ImageUsageFlags,
    pub name: String,
}

impl Image {
    pub fn new(base: &Base, settings: &ImageSettings) -> Result<Self, UrnError> {
        let image_info = ash::vk::ImageCreateInfo::builder()
            .image_type(ash::vk::ImageType::TYPE_2D)
            .extent(ash::vk::Extent3D {
                width: settings.width,
                height: settings.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .format(settings.format)
            .tiling(settings.tiling)
            .initial_layout(ash::vk::ImageLayout::UNDEFINED)
            .usage(settings.usage)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .samples(ash::vk::SampleCountFlags::TYPE_1);

        let image = unsafe { base.logical_device.0.create_image(&image_info, None)? };
        base.name_object(image, settings.name.clone())?;

        Ok(Self(image))
    }
}
