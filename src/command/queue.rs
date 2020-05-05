use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Queue(pub ash::vk::Queue);

impl Queue {
    pub fn new(base: &Base, queue_family_index: u32, queue_index: u32, name: String) -> Result<Self, UrnError> {
        let queue = unsafe {
            base.logical_device
                .0
                .get_device_queue(queue_family_index, queue_index)
        };
        base.name_object(queue, name)?;

        Ok(Self(queue))
    }
}
