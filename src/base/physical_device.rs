use crate::error::UrnError;

use ash::version::InstanceV1_0;

pub struct PhysicalDevice(pub ash::vk::PhysicalDevice);

impl PhysicalDevice {
    pub fn enumerate(instance: &ash::Instance) -> Result<Vec<Self>, UrnError> {
        let physical_devices = unsafe { instance.enumerate_physical_devices()? };
        Ok(physical_devices
            .iter()
            .map(|i| PhysicalDevice(*i))
            .collect())
    }
}
