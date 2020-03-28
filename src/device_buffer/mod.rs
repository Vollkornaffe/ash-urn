use crate::UrnError;
use crate::Base;

mod buffer;
mod memory;

pub use buffer::Buffer;
pub use memory::Memory;

use buffer::BufferSettings;
use memory::MemorySettings;


pub struct DeviceBuffer {
    pub buffer: Buffer,
    pub memory: Memory,
}

pub struct DeviceBufferSettings {
    pub size: ash::vk::DeviceSize,
    pub usage: ash::vk::BufferUsageFlags,
    pub properties: ash::vk::MemoryPropertyFlags,
    pub name: String,
}

impl DeviceBuffer {

    pub fn new(
        base: &Base,
        settings: &DeviceBufferSettings,
    ) -> Result<Self, UrnError> {
        let buffer = Buffer::new(
            base,
            &BufferSettings {
                size: settings.size,
                usage: settings.usage,
                name: format!("{}Buffer", settings.name.clone()),
            },
        )?;

        let memory = Memory::alloc(
            base,
            &MemorySettings {
                properties: settings.properties,
                buffer: buffer.0,
                name: format!("{}Memory", settings.name.clone()),
            },
        )?;

        Ok(Self {
            buffer,
            memory,
        })
    }
}
