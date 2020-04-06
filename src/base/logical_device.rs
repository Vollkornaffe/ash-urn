use crate::util::StringContainer;
use crate::UrnError;

use ash::version::InstanceV1_0;

pub struct LogicalDevice(pub ash::Device);

pub struct LogicalDeviceSettings {
    pub extension_names: Vec<String>,
    pub enable_validation: bool,
    pub validation_layer_names: Vec<String>,
    pub queues: Vec<u32>,
    pub timelines: bool,
}

impl LogicalDevice {
    pub fn new(
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        settings: LogicalDeviceSettings,
    ) -> Result<Self, UrnError> {
        let mut queue_create_infos = vec![];
        let queue_priority = [1.0_f32];
        for &queue_idx in settings.queues.iter() {
            let queue_create_info = ash::vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_idx)
                .queue_priorities(&queue_priority)
                .build();
            queue_create_infos.push(queue_create_info);
        }

        let extension_names_cs = StringContainer::new(settings.extension_names.clone());

        let mut timeline_feature = ash::vk::PhysicalDeviceTimelineSemaphoreFeatures::builder()
            .timeline_semaphore(true)
            .build();
        let next_ptr = &mut timeline_feature
            as *mut ash::vk::PhysicalDeviceTimelineSemaphoreFeatures
            as *mut ash::vk::BaseOutStructure;
        let mut physical_device_features_2 = ash::vk::PhysicalDeviceFeatures2::builder().build();
        physical_device_features_2.p_next = next_ptr as _;

        let device_create_info = ash::vk::DeviceCreateInfo::builder()
            .queue_create_infos(queue_create_infos.as_slice())
            .enabled_extension_names(extension_names_cs.pointer.as_slice());

        let device_create_info = if settings.timelines {
            device_create_info.push_next(&mut physical_device_features_2)
        } else {
            device_create_info
        };

        let validation_layer_names_cs =
            StringContainer::new(settings.validation_layer_names.clone());
        let device_create_info = if settings.enable_validation {
            device_create_info.enabled_layer_names(validation_layer_names_cs.pointer.as_slice())
        } else {
            device_create_info
        };

        let logical_device =
            unsafe { instance.create_device(physical_device, &device_create_info, None)? };

        Ok(Self(logical_device))
    }
}
