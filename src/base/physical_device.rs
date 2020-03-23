use crate::error::UrnError;
use crate::util::vk_to_string;

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

    pub fn check_extensions(
        &self,
        instance: &ash::Instance,
        extension_names: &[&str],
    ) -> Result<Vec<bool>, UrnError> {
        let available_extensions: Vec<String> =
            unsafe { instance.enumerate_device_extension_properties(self.0)? }
                .iter()
                .map(|e| vk_to_string(&e.extension_name))
                .collect();
        Ok(extension_names
            .iter()
            .map(|e| available_extensions.contains(&e.to_string()))
            .collect())
    }
}
