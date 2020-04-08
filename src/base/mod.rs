use crate::UrnError;

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

use std::ops::BitAnd;

/// Very basic setup for a vulkan app.
pub struct Base {
    pub entry: Entry,
    pub instance: Instance,
    pub validation: Option<Validation>,
    pub physical_device: PhysicalDevice,
    pub logical_device: LogicalDevice,
    pub timeline_loader: ash::extensions::khr::TimelineSemaphore,
    pub queue_map: std::collections::HashMap<QueueFamilyKey, QueueFamily>,
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

    pub fn find_memory_type_index(
        &self,
        memory_type_bits: ash::vk::MemoryPropertyFlags,
        required_properties: ash::vk::MemoryPropertyFlags,
    ) -> Result<u32, UrnError> {
        let memory_properties = unsafe {
            self.instance
                .0
                .get_physical_device_memory_properties(self.physical_device.0)
        };
        for i in 0..memory_properties.memory_type_count {
            if memory_type_bits.bitand(ash::vk::MemoryPropertyFlags::from_raw(1 << i))
                == ash::vk::MemoryPropertyFlags::from_raw(1 << i)
                && memory_properties.memory_types[i as usize]
                    .property_flags
                    .bitand(required_properties)
                    == required_properties
            {
                return Ok(i);
            }
        }
        Err(UrnError::Generic("failed to find suitable memory!"))
    }

    pub fn find_supported_format(
        &self,
        candidates: Vec<ash::vk::Format>,
        tiling: ash::vk::ImageTiling,
        features: ash::vk::FormatFeatureFlags,
    ) -> Result<ash::vk::Format, UrnError> {
        for format in candidates {
            let properties = unsafe {
                self.instance
                    .0
                    .get_physical_device_format_properties(self.physical_device.0, format)
            };

            match tiling {
                ash::vk::ImageTiling::LINEAR => {
                    if properties.linear_tiling_features.bitand(features) == features {
                        return Ok(format);
                    }
                }
                ash::vk::ImageTiling::OPTIMAL => {
                    if properties.optimal_tiling_features.bitand(features) == features {
                        return Ok(format);
                    }
                }
                _ => {}
            }
        }
        Err(UrnError::Generic("Format is not supported"))
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
