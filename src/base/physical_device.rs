use crate::error::UrnError;
use crate::util::vk_to_string;

use ash::version::InstanceV1_0;

use super::{QueueFamily, QueueFamilyKey};

pub struct PhysicalDevice(pub ash::vk::PhysicalDevice);

pub struct SwapChainSupportDetail {
    pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<ash::vk::SurfaceFormatKHR>,
    pub present_modes: Vec<ash::vk::PresentModeKHR>,
}

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

    pub fn query_swapchain_support(
        &self,
        surface_loader: &ash::extensions::khr::Surface,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<SwapChainSupportDetail, UrnError> {
        unsafe {
            let capabilities =
                surface_loader.get_physical_device_surface_capabilities(self.0, surface)?;
            let formats = surface_loader.get_physical_device_surface_formats(self.0, surface)?;
            let present_modes =
                surface_loader.get_physical_device_surface_present_modes(self.0, surface)?;

            Ok(SwapChainSupportDetail {
                capabilities,
                formats,
                present_modes,
            })
        }
    }

    pub fn query_queues(
        &self,
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<std::collections::HashMap<QueueFamilyKey, QueueFamily>, UrnError> {
        let mut res = std::collections::HashMap::new();
        let available_queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(self.0) };
        for (idx, properties) in available_queue_families.iter().enumerate() {
            if properties.queue_count == 0 {
                continue;
            }
            let flags = properties.queue_flags;
            res.insert(
                QueueFamilyKey {
                    graphics: flags.contains(ash::vk::QueueFlags::GRAPHICS),
                    present: unsafe {
                        surface_loader
                            .get_physical_device_surface_support(self.0, idx as u32, surface)?
                    },
                    transfer: flags.contains(ash::vk::QueueFlags::TRANSFER),
                    compute: flags.contains(ash::vk::QueueFlags::COMPUTE),
                },
                QueueFamily {
                    idx: idx as u32,
                    properties: *properties,
                },
            );
        }
        Ok(res)
    }
}
