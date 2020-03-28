use crate::Base;
use crate::UrnError;

pub mod buffer;
pub mod pool;
pub mod queue;
pub mod single_time;

pub use buffer::Buffer;
pub use pool::Pool;
pub use queue::Queue;

pub struct Command {
    queue: Queue,
    pool: Pool,
    buffers: Vec<Buffer>,
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
            queue,
            pool,
            buffers,
        })
    }
}
