use crate::error::UrnError;
use crate::util::vk_to_string;

use ash::version::EntryV1_0;

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> ash::vk::Bool32 {
    let severity = match message_severity {
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    ash::vk::FALSE
}

pub fn populate_debug_messenger_create_info() -> ash::vk::DebugUtilsMessengerCreateInfoEXT {
    ash::vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                //| ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        )
        .pfn_user_callback(Some(vulkan_debug_utils_callback))
        .build()
}

pub fn check_validation_layer_support(
    validation_layer_names: Vec<String>,
    ash_entry: &ash::Entry,
) -> Result<(), UrnError> {
    let layer_properties = ash_entry.enumerate_instance_layer_properties()?;

    if layer_properties.is_empty() {
        return Err(UrnError::Generic("No available layers."));
    }

    for layer_needed in validation_layer_names.iter() {
        let mut is_layer_found = false;
        for layer in layer_properties.iter() {
            if *layer_needed == vk_to_string(&layer.layer_name) {
                is_layer_found = true;
                break;
            }
        }

        if !is_layer_found {
            return Err(UrnError::Generic("Validation layer not found."));
        }
    }

    Ok(())
}

pub struct Validation {
    pub debug_utils_loader: ash::extensions::ext::DebugUtils,
    pub debug_messenger: ash::vk::DebugUtilsMessengerEXT,
}

impl Validation {
    pub fn new(ash_entry: &ash::Entry, ash_instance: &ash::Instance) -> Result<Self, UrnError> {
        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(ash_entry, ash_instance);

        let debug_utils_messenger_create_info = populate_debug_messenger_create_info();
        let debug_messenger = unsafe {
            debug_utils_loader
                .create_debug_utils_messenger(&debug_utils_messenger_create_info, None)?
        };
        Ok(Self {
            debug_utils_loader,
            debug_messenger,
        })
    }
}

impl Drop for Validation {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils_loader.destroy_debug_utils_messenger(self.debug_messenger, None);
        }
    }
}
