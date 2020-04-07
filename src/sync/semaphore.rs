use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Semaphore(pub ash::vk::Semaphore);

impl Semaphore {
    pub fn new(base: &Base, name: String) -> Result<Self, UrnError> {
        let semaphore_info = ash::vk::SemaphoreCreateInfo::builder();
        let semaphore = unsafe {
            base.logical_device
                .0
                .create_semaphore(&semaphore_info, None)?
        };
        base.name_object(semaphore, name)?;

        Ok(Self(semaphore))
    }
    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_semaphore(self.0, None);        
        }
    }
}
