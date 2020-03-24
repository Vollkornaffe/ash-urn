use crate::error::UrnError;

pub mod entry;
pub mod instance;
pub mod logical_device;
pub mod physical_device;
pub mod queue_families;
pub mod swapchain;
pub mod validation;

pub use entry::Entry;
pub use instance::{Instance, InstanceSettings};
pub use logical_device::{LogicalDevice, LogicalDeviceSettings};
pub use physical_device::{PhysicalDevice, PhysicalDeviceSettings};
pub use queue_families::{QueueFamily, QueueFamilyKey};
pub use swapchain::SwapChainSupportDetail;
pub use validation::Validation;

use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;

/// Very basic setup for a vulkan app.
pub struct Base {
    pub entry: Entry,
    pub instance: Instance,
    pub validation: Option<Validation>,
    pub physical_device: PhysicalDevice,
    pub logical_device: LogicalDevice,
}

impl Base {
    pub fn name_object<T: ash::vk::Handle>(
        &self,
        ash_object: T,
        name: String,
    ) -> Result<(), UrnError> {
        match &self.validation {
            Some(v) => v.name_object(&self.logical_device.0, ash_object, name),
            None => Ok(()),
        }
    }
}


impl Drop for Base {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.0.destroy_device(None);
            match &self.validation {
                Some(v) => v
                    .debug_utils_loader
                    .destroy_debug_utils_messenger(v.debug_messenger, None),
                None => {}
            }
            self.instance.0.destroy_instance(None);
        }
    }
}
