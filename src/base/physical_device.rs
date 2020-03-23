use crate::error::UrnError;
use crate::util::vk_to_string;

use ash::version::InstanceV1_0;
use ash::version::InstanceV1_1;

use super::SwapChainSupportDetail;
use super::{QueueFamily, QueueFamilyKey,};
use super::queue_families::{COMBINED, DEDICATED_TRANSFER,};

pub struct PhysicalDevice(pub ash::vk::PhysicalDevice);

pub struct PhysicalDeviceSettings {
    pub timelines: bool,
    pub subgroups: bool,
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
        extension_names: Vec<String>,
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
            let queue_family = QueueFamily {
                idx: idx as u32,
                properties: *properties,
            };
            res.insert(
                QueueFamilyKey::gen_key(
                    &queue_family,
                    self.0,
                    instance,
                    surface_loader,
                    surface,
                )?,
                queue_family,
            );
        }
        Ok(res)
    }

    pub fn check_timeline_feature(&self, instance: &ash::Instance) -> bool {
        let mut timeline_feature = ash::vk::PhysicalDeviceTimelineSemaphoreFeatures::builder()
            .timeline_semaphore(false)
            .build();
        let mut physical_device_features2 = ash::vk::PhysicalDeviceFeatures2::builder().build();
        let next_ptr = &mut timeline_feature
            as *mut ash::vk::PhysicalDeviceTimelineSemaphoreFeatures
            as *mut ash::vk::BaseOutStructure;
        physical_device_features2.p_next = next_ptr as _;
        unsafe { instance.get_physical_device_features2(self.0, &mut physical_device_features2) };
        timeline_feature.timeline_semaphore != 0
    }

    pub fn query_subgroup_properties(
        &self,
        instance: &ash::Instance,
    ) -> ash::vk::PhysicalDeviceSubgroupProperties {
        let mut subgroup_properties = ash::vk::PhysicalDeviceSubgroupProperties::builder().build();
        let mut physical_device_properties2 = ash::vk::PhysicalDeviceProperties2::builder().build();
        let next_ptr = &mut subgroup_properties as *mut ash::vk::PhysicalDeviceSubgroupProperties
            as *mut ash::vk::BaseOutStructure;
        physical_device_properties2.p_next = next_ptr as _;
        unsafe {
            instance.get_physical_device_properties2(self.0, &mut physical_device_properties2)
        };
        subgroup_properties
    }

    pub fn print_details(
        &self,
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<(), UrnError> {
        let device_properties = unsafe { instance.get_physical_device_properties(self.0) };
        let device_features = unsafe { instance.get_physical_device_features(self.0) };
        let device_queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(self.0) };

        let device_type = match device_properties.device_type {
            ash::vk::PhysicalDeviceType::CPU => "Cpu",
            ash::vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            ash::vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            ash::vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
            ash::vk::PhysicalDeviceType::OTHER => "Unknown",
            _ => panic!(),
        };

        let device_name = vk_to_string(&device_properties.device_name);
        println!(
            "\tDevice Name: {}, id: {}, type: {}",
            device_name, device_properties.device_id, device_type
        );

        let major_version = ash::vk::version_major(device_properties.api_version);
        let minor_version = ash::vk::version_minor(device_properties.api_version);
        let patch_version = ash::vk::version_patch(device_properties.api_version);

        println!(
            "\tAPI Version: {}.{}.{}",
            major_version, minor_version, patch_version
        );

        println!("\tSupport Queue Family: {}", device_queue_families.len());
        println!("\t\tQueue Count | Graphics, Present, Transfer, Compute");
        for (idx,properties) in device_queue_families.iter().enumerate() {
            let queue_family = QueueFamily {
                idx: idx as u32,
                properties: *properties,
            };
            let key = QueueFamilyKey::gen_key(
                &queue_family,
                self.0,
                instance,
                surface_loader,
                surface,
            )?;
            let support_string = |b| if b {
                "support"
            } else {
                "unsupport"
            };
            println!(
                "\t\t{}\t    | {},  {},  {},  {}",
                properties.queue_count,
                support_string(key.graphics),
                support_string(key.present),
                support_string(key.transfer),
                support_string(key.compute),
            );
        }
        Ok(())
    }

    pub fn print_limits(&self, instance: &ash::Instance) {
        let device_properties = unsafe { instance.get_physical_device_properties(self.0) };
        println!("{:?}", device_properties.limits,);
    }

    /// Optional function to pick a GPU
    pub fn pick_gpu(
        instance: &ash::Instance,
        device_extensions: Vec<String>,
        surface_loader: &ash::extensions::khr::Surface,
        surface: ash::vk::SurfaceKHR,
        settings: PhysicalDeviceSettings,
    ) -> Result<Self, UrnError> {
        let physical_devices = Self::enumerate(&instance)?;
        for pd in physical_devices {
            pd.print_details(&instance, surface_loader, surface);
            let mut device_ok = true;

            for (i, b) in pd
                .check_extensions(&instance, device_extensions.clone())
                .unwrap()
                .iter()
                .enumerate()
            {
                if !b {
                    println!(
                        "The following extension was not found: {}",
                        device_extensions[i]
                    );
                    device_ok = false;
                }
            }

            let swapchain_support = pd.query_swapchain_support(surface_loader, surface)?;

            if swapchain_support.formats.is_empty() {
                println!("Found no supported swapchain formats.");
                device_ok = false;
            }

            if swapchain_support.present_modes.is_empty() {
                println!("Found no supported present modes.");
                device_ok = false;
            }

            let queue_map = pd.query_queues(
                instance,
                surface_loader,
                surface,
            )?;
            if !queue_map.contains_key(&COMBINED) {
                println!("Found no combined queue family.");
                device_ok = false;
            }
            if !queue_map.contains_key(&DEDICATED_TRANSFER) {
                println!("Found no dedicated transfer queue family.");
                device_ok = false;
            }

            if settings.timelines && !pd.check_timeline_feature(instance) {
                println!("Timeline not available.");
                device_ok = false;
            }

            if settings.subgroups {
                let subgroup_properties = pd.query_subgroup_properties(
                    instance,
                );
                if !subgroup_properties.supported_stages.contains(
                    ash::vk::ShaderStageFlags::COMPUTE
                ) {
                    println!("Subgroup not supported in compute shader.");
                    device_ok = false;
                }
                if !subgroup_properties.supported_operations.contains(
                ash::vk::SubgroupFeatureFlags::BASIC
                ) {
                    println!("Subgroup basic not supported.");
                    device_ok = false;
                }
                if !subgroup_properties.supported_operations.contains(
                ash::vk::SubgroupFeatureFlags::ARITHMETIC
                ) {
                    println!("Subgroup artihmetic not supported.");
                    device_ok = false;
                }
                if !subgroup_properties.supported_operations.contains(
                ash::vk::SubgroupFeatureFlags::BALLOT
                ) {
                    println!("Subgroup ballot not supported.");
                    device_ok = false;
                }
            }

            if device_ok {
                println!("Found suitable device!");
                return Ok(pd);
            } else {
                println!("This device was not suitable.");
            }
        }
        Err(UrnError::Generic("Could not find a suitable device!"))
    }
}
