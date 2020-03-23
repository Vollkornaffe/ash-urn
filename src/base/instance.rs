use super::validation::{check_validation_layer_support, populate_debug_messenger_create_info};
use crate::error::UrnError;
use crate::util::CString;
use crate::util::StringContainer;

use ash::version::EntryV1_0;

/// Shallow wapper
pub struct Instance(pub ash::Instance);

pub struct InstanceSettings {
    pub name: String,
    pub version_major: u32,
    pub version_minor: u32,
    pub version_patch: u32,
    pub extension_names: Vec<String>,
    pub enable_validation: bool,
    pub validation_layer_names: Vec<String>,
}

impl Instance {
    pub fn new(settings: InstanceSettings, entry: &ash::Entry) -> Result<Instance, UrnError> {
        if settings.enable_validation {
            check_validation_layer_support(settings.validation_layer_names.clone(), entry)?;
        }

        let name_buf = CString::new(settings.name)?;

        let app_info = ash::vk::ApplicationInfo::builder()
            .api_version(ash::vk::make_version(
                settings.version_major,
                settings.version_minor,
                settings.version_patch,
            ))
            .application_name(&name_buf);

        let extension_names_cs = StringContainer::new(settings.extension_names.clone());
        let create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(extension_names_cs.pointer.as_slice());

        let validation_layer_names_cs =
            StringContainer::new(settings.validation_layer_names.clone());

        let mut debug_utils_messenger_create_info = populate_debug_messenger_create_info();

        let create_info = if settings.enable_validation {
            create_info
                .enabled_layer_names(validation_layer_names_cs.pointer.as_slice())
                .push_next(&mut debug_utils_messenger_create_info)
        } else {
            create_info
        };

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok(Instance(instance))
    }
}
