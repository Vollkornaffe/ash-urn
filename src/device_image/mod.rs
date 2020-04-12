use crate::Base;
use crate::UrnError;

pub mod depth;
pub mod image;
pub mod memory;
pub mod sampler;
pub mod view;

pub use self::image::Image;
pub use depth::create_depth_device_image;
pub use memory::Memory;
pub use sampler::Sampler;
pub use view::View;

pub use self::image::ImageSettings;
pub use memory::MemorySettings;
pub use view::ViewSettings;

use ash::version::DeviceV1_0;

pub struct DeviceImage {
    pub image: Image,
    pub memory: Memory,
    pub view: View,
}

pub struct DeviceImageSettings {
    pub width: u32,
    pub height: u32,
    pub format: ash::vk::Format,
    pub tiling: ash::vk::ImageTiling,
    pub usage: ash::vk::ImageUsageFlags,
    pub properties: ash::vk::MemoryPropertyFlags,
    pub aspect_flags: ash::vk::ImageAspectFlags,
    pub name: String,
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
            base.logical_device.0.destroy_image_view(self.view.0, None);
            base.logical_device.0.destroy_image(self.image.0, None);
            base.logical_device.0.free_memory(self.memory.0, None);
        }
    }
}
