use crate::Base;
use crate::UrnError;

pub mod image;
pub mod memory;
pub mod view;

pub use image::Image;
pub use memory::Memory;
pub use view::View;

use image::ImageSettings;
use memory::MemorySettings;
use view::ViewSettings;

use ash::version::DeviceV1_0;

pub struct DeviceImage {
    pub image: Image,
    pub memory: Memory,
    pub view: View,
}

pub struct DeviceImageSettings {
    width: u32,
    height: u32,
    format: ash::vk::Format,
    tiling: ash::vk::ImageTiling,
    usage: ash::vk::ImageUsageFlags,
    properties: ash::vk::MemoryPropertyFlags,
    aspect_flags: ash::vk::ImageAspectFlags,
    name: String,
}

impl DeviceImage {
    pub fn new(base: &Base, settings: &DeviceImageSettings) -> Result<Self, UrnError> {
        let image = Image::new(
            base,
            &ImageSettings {
                width: settings.width,
                height: settings.height,
                format: settings.format,
                tiling: settings.tiling,
                usage: settings.usage,
                name: format!("{}Image", settings.name.clone()),
            },
        )?;

        let memory = Memory::alloc(
            base,
            &MemorySettings {
                properties: settings.properties,
                image: image.0,
                name: format!("{}Memory", settings.name.clone()),
            },
        )?;

        let view = View::new(
            base,
            &ViewSettings {
                image: image.0,
                format: settings.format,
                aspect_flags: settings.aspect_flags,
                name: format!("{}View", settings.name.clone()),
            },
        )?;

        Ok(Self {
            image,
            memory,
            view,
        })
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0
                .destroy_image_view(self.view.0, None);
            base.logical_device.0.destroy_image(self.image.0, None);
            base.logical_device.0
                .free_memory(self.memory.0, None);
        }
    }
}
