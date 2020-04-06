use crate::Base;
use crate::UrnError;

mod buffer;
mod memory;

pub use buffer::Buffer;
pub use memory::Memory;

use buffer::BufferSettings;
use memory::MemorySettings;

use ash::version::DeviceV1_0;

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
    pub fn new(base: &Base, settings: &DeviceBufferSettings) -> Result<Self, UrnError> {
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

        Ok(Self { buffer, memory })
    }

    pub fn write<T>(&self, base: &Base, to_write: T) -> Result<(), UrnError> {
        let size = std::mem::size_of::<T>() as ash::vk::DeviceSize;

        let data_ptr = unsafe {
            base.logical_device.0.map_memory(
                self.memory.0,
                0,
                size,
                ash::vk::MemoryMapFlags::default(),
            )?
        } as *mut T;

        unsafe {
            data_ptr.copy_from_nonoverlapping(&to_write, 1);
            base.logical_device.0.unmap_memory(self.memory.0)
        };

        Ok(())
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_buffer(self.buffer.0, None);
            base.logical_device.0.free_memory(self.memory.0, None);
        }
    }
}
