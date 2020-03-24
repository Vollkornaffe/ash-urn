mod buffer;
mod memory;

pub use buffer::Buffer;
pub use memory::Memory;

pub struct DeviceBuffer {
    pub buffer: Buffer,
    pub memory: Memory,
}
