use crate::AppError;
use crate::SDL;

const ENABLE_VALIDATION: bool = cfg!(debug_assertions);

use ash_urn::base::{
    Base, Entry, Instance, InstanceSettings, LogicalDevice, LogicalDeviceSettings, PhysicalDevice,
    PhysicalDeviceSettings, Validation,
};

pub fn setup(
    sdl: &mut SDL,
) -> Result<(Base, ash::extensions::khr::Surface, ash::vk::SurfaceKHR), AppError> {
    // Get our requriements ready
    let mut instance_extension_names = sdl.required_extension_names()?;
    instance_extension_names.push(
        ash::extensions::ext::DebugUtils::name()
            .to_str()
            .unwrap()
            .to_string(),
    );
    instance_extension_names.push("VK_KHR_get_physical_device_properties2".to_string());
    let validation_layer_names = vec!["VK_LAYER_KHRONOS_validation".to_string()];

    let entry = Entry::new()?;
    // Instance needs vulkan version
    let instance = Instance::new(
        InstanceSettings {
            name: "Test".to_string(),
            version_major: 1,
            version_minor: 2,
            version_patch: 131,
            extension_names: instance_extension_names,
            enable_validation: ENABLE_VALIDATION,
            validation_layer_names: validation_layer_names.clone(),
        },
        &entry.0,
    )?;
    // Get our input validated!
    let validation = if ENABLE_VALIDATION {
        Some(Validation::new(&entry.0, &instance.0)?)
    } else {
        None
    };
    // Ready for the surface to draw on
    let surface_loader = ash::extensions::khr::Surface::new(&entry.0, &instance.0);
    let surface = sdl.create_surface(&instance.0)?;

    // Time to think about devices
    let timelines = true;
    let mut device_extensions = vec!["VK_KHR_swapchain".to_string()];
    if timelines {
        device_extensions.push("VK_KHR_timeline_semaphore".to_string());
    }
    // First get the actual gpu
    let physical_device = PhysicalDevice::pick_gpu(
        &instance.0,
        device_extensions.clone(),
        &surface_loader,
        surface,
        PhysicalDeviceSettings {
            timelines,
            subgroups: true,
        },
    )?;

    let queue_map = physical_device.query_queues(&instance.0, &surface_loader, surface)?;
    let transfer_queue_family_idx = queue_map
        .get(&ash_urn::base::queue_families::DEDICATED_TRANSFER)
        .unwrap()
        .idx;
    let combined_queue_family_idx = queue_map
        .get(&ash_urn::base::queue_families::COMBINED)
        .unwrap()
        .idx;

    // Then the logical device that does all of the heavy lifting
    let logical_device = LogicalDevice::new(
        &instance.0,
        physical_device.0,
        LogicalDeviceSettings {
            extension_names: device_extensions,
            enable_validation: ENABLE_VALIDATION,
            validation_layer_names: validation_layer_names.clone(),
            queues: vec![transfer_queue_family_idx, combined_queue_family_idx],
            timelines,
        },
    )?;

    let timeline_loader = ash::extensions::khr::TimelineSemaphore::new(&entry.0, &instance.0);

    // Combine everything into the Base
    let base = Base {
        entry,
        instance,
        validation,
        physical_device,
        logical_device,
        timeline_loader,
        queue_map,
    };

    Ok((base, surface_loader, surface))
}
