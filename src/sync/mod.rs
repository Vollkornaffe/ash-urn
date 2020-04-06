use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub mod semaphore;
pub mod timeline;

pub use semaphore::Semaphore;
pub use timeline::Timeline;

pub fn wait_device_idle(base: &Base) -> Result<(), UrnError> {
    unsafe { base.logical_device.0.device_wait_idle()? }
    Ok(())
}
