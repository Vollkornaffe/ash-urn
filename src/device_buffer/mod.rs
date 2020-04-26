use crate::Base;
use crate::UrnError;

use crate::command::single_time;

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
    pub size: ash::vk::DeviceSize,
    pub shared: bool,
}

pub struct DeviceBufferSettings {
    pub size: ash::vk::DeviceSize,
    pub usage: ash::vk::BufferUsageFlags,
    pub properties: ash::vk::MemoryPropertyFlags,
    pub shared: bool,
    pub name: String,
}

impl DeviceBuffer {
    pub fn new(base: &Base, settings: &DeviceBufferSettings) -> Result<Self, UrnError> {
        let buffer = Buffer::new(
            base,
            &BufferSettings {
                size: settings.size,
                usage: settings.usage,
                shared: settings.shared,
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

        Ok(Self { buffer, memory, size: settings.size, shared: settings.shared})
    }

    pub fn write<T>(&self, base: &Base, to_write: &T) -> Result<(), UrnError> {

        let data_ptr = unsafe {
            base.logical_device.0.map_memory(
                self.memory.0,
                0,
                self.size,
                ash::vk::MemoryMapFlags::default(),
            )?
        } as *mut T;

        unsafe {
            data_ptr.copy_from_nonoverlapping(to_write, 1);
            base.logical_device.0.unmap_memory(self.memory.0)
        };

        Ok(())
    }

    pub fn read<T>(&self, base: &Base, to_read: &mut T) -> Result<(), UrnError> {

        let data_ptr = unsafe {
            base.logical_device.0.map_memory(
                self.memory.0,
                0,
                self.size,
                ash::vk::MemoryMapFlags::default(),
            )?
        } as *mut T;

        unsafe {
            (to_read as *mut T).copy_from_nonoverlapping(data_ptr, 1);
            base.logical_device.0.unmap_memory(self.memory.0)
        };

        Ok(())
    }

    pub fn write_slice<T>(&self, base: &Base, to_write: &[T]) -> Result<(), UrnError> {

        let data_ptr = unsafe {
            base.logical_device.0.map_memory(
                self.memory.0,
                0,
                self.size,
                ash::vk::MemoryMapFlags::default(),
            )?
        } as *mut T;

        unsafe {
            data_ptr.copy_from_nonoverlapping(to_write.as_ptr(), to_write.len());
            base.logical_device.0.unmap_memory(self.memory.0)
        };

        Ok(())
    }

    pub fn read_slice<T>(&self, base: &Base, to_read: &mut [T]) -> Result<(), UrnError> {

        let data_ptr = unsafe {
            base.logical_device.0.map_memory(
                self.memory.0,
                0,
                self.size,
                ash::vk::MemoryMapFlags::default(),
            )?
        } as *mut T;

        unsafe {
            (to_read.as_mut_ptr()).copy_from_nonoverlapping(data_ptr, to_read.len());
            base.logical_device.0.unmap_memory(self.memory.0)
        };

        Ok(())
    }

    pub fn set_zero<T>(
        &self,
        base: &Base,
        queue: ash::vk::Queue,
        pool: ash::vk::CommandPool,
    ) -> Result<(), UrnError> {
        
        let command_buffer = single_time::begin(base, pool, "SetBufferToZero".to_string())?;

        unsafe {
            base.logical_device.0.cmd_fill_buffer(
                command_buffer,
                self.buffer.0,
                0,
                self.size,
                0,
            )
        };

        single_time::end(base, queue, pool, command_buffer)

    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_buffer(self.buffer.0, None);
            base.logical_device.0.free_memory(self.memory.0, None);
        }
    }
}
