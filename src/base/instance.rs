use super::validation::{
    check_validation_layer_support, populate_debug_messenger_create_info,
};
use crate::error::UrnError;
use crate::util::CString;
use crate::util::StringContainer;

use ash::version::EntryV1_0;

/// Shallow wapper
pub struct Instance(pub ash::Instance);

impl Instance {
    pub fn new(
        name: &str,
        version_major: u32,
        version_minor: u32,
        version_patch: u32,
        extension_names: &[&str],
        enable_validation: bool,
        validation_layer_names: &[&str],
        entry: &ash::Entry,
    ) -> Result<Instance, UrnError> {
        if enable_validation {
            check_validation_layer_support(
                validation_layer_names,
                entry)?;
        }

        let name_buf = CString::new(name)?;

        let app_info = ash::vk::ApplicationInfo::builder()
            .api_version(ash::vk::make_version(
                version_major,
                version_minor,
                version_patch,
            ))
            .application_name(&name_buf);

        let extension_names_cs = StringContainer::new(extension_names);
        let create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(extension_names_cs.pointer.as_slice());

        let validation_layer_names_cs =
            StringContainer::new(validation_layer_names);

        let mut debug_utils_messenger_create_info = populate_debug_messenger_create_info();

        let create_info = if enable_validation {
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
