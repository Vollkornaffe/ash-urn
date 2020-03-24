use crate::error::UrnError;
use crate::base::Base;

use ash::version::DeviceV1_0;

pub struct View(pub ash::vk::ImageView);

pub struct ViewSettings {
    image: ash::vk::Image,
    format: ash::vk::Format,
    aspect_flags: ash::vk::ImageAspectFlags,
    name: String,
}

impl View {
    pub fn new(
        base: &Base,
        settings: &ViewSettings,
    ) -> Result<Self, UrnError> {
        let view_info = ash::vk::ImageViewCreateInfo::builder()
            .image(settings.image)
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(settings.format)
            .subresource_range(
                ash::vk::ImageSubresourceRange::builder()
                    .aspect_mask(settings.aspect_flags)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            );
        let image_view = unsafe { base.logical_device.0.create_image_view(&view_info, None)? };
        base.name_object(image_view, settings.name.clone())?;

        Ok(Self(image_view))
    }
}
