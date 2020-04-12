use crate::Base;
use crate::UrnError;

pub mod buffer;
pub mod draw;
pub mod pool;
pub mod queue;
pub mod single_time;
pub mod image_layout;

pub use buffer::Buffer;
pub use draw::DrawIndexedSettings;
pub use image_layout::TransitionImageLayoutSettings;
pub use pool::Pool;
pub use queue::Queue;

use ash::version::DeviceV1_0;

pub struct Command {
    pub family_idx: u32,
    pub queue: Queue,
    pub pool: Pool,
    pub buffers: Vec<Buffer>,
}

pub struct CommandSettings {
    pub queue_family_idx: u32,
    pub n_buffer: u32,
    pub name: String,
}

impl Command {
    pub fn new(base: &Base, settings: &CommandSettings) -> Result<Self, UrnError> {
        let queue = Queue::new(
            &base,
            settings.queue_family_idx,
            format!("{}Queue", settings.name.clone()),
        )?;

        let pool = Pool::new(
            &base,
            settings.queue_family_idx,
            format!("{}Pool", settings.name.clone()),
        )?;

        let buffers = Buffer::alloc_vec(
            &base,
            pool.0,
            settings.n_buffer,
            format!("{}Buffer", settings.name.clone()),
        )?;

        Ok(Self {
            family_idx: settings.queue_family_idx,
            queue,
            pool,
            buffers,
        })
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device
                .0
                .destroy_command_pool(self.pool.0, None);
        }
    }
}
