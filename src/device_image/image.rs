use crate::error::UrnError;
use crate::base::Base;

use ash::version::DeviceV1_0;

pub struct Image(pub ash::vk::Image);

pub struct ImageSettings {
    width: u32,
    height: u32,
    format: ash::vk::Format,
    tiling: ash::vk::ImageTiling,
    usage: ash::vk::ImageUsageFlags,
    sharing_mode: ash::vk::SharingMode,
    name: String,
}

impl Image {
    pub fn new(
        base: &Base,
        settings: &ImageSettings,
    ) -> Result<Self, UrnError> {
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
            .sharing_mode(settings.sharing_mode)
            .samples(ash::vk::SampleCountFlags::TYPE_1);

        let image = unsafe { base.logical_device.0.create_image(&image_info, None)? };
        base.name_object(image, settings.name.clone())?;

        Ok(Self(image))
    }
}
