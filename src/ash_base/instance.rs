/// Shallow wapper
pub struct Instance(ash::Instance);

impl Instance {
    pub fn new(
        name: &str,
        version_major: u32,
        version_minor: u32,
        version_patch: u32,
        extension_names: &[&str],
        entry: &ash::Entry,
    ) -> Result<Instance, ash::InstanceError> {
        /*
        if debug::VALIDATION.is_enable {
            debug::check_validation_layer_support(entry)?;
        }

        let name_buf = CString::new(name)?;

        let app_info = ash::vk::ApplicationInfo::builder()
            .api_version(ash::vk::make_version(
                version_major,
                version_minor,
                version_patch,
            ))
            .application_name(&name_buf);


        let all_extension_name_bufs: Vec<CString> = extension_names
            .iter()
            .chain(device_creation::INSTANCE_EXTENSIONS.names)
            .map(|name| CString::new(*name))
            .collect::<Result<_, _>>()?;
        let all_extension_name_ptrs: Vec<*const i8> = all_extension_name_bufs
            .iter()
            .map(|name| name.as_ptr())
            .collect();

        let create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(all_extension_name_ptrs.as_slice());

        let required_validation_layer_name_bufs: Vec<CString> = debug::VALIDATION
            .required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name))
            .collect::<Result<_, _>>()?;
        let required_validation_layer_name_ptrs: Vec<*const std::os::raw::c_char> =
            required_validation_layer_name_bufs
                .iter()
                .map(|name| name.as_ptr())
                .collect();

        let mut debug_utils_messenger_create_info = debug::populate_debug_messenger_create_info();

        let create_info = if debug::VALIDATION.is_enable {
            create_info
                .enabled_layer_names(required_validation_layer_name_ptrs.as_slice())
                .push_next(&mut debug_utils_messenger_create_info)
        } else {
            create_info
        };

        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(ash::InstanceError)?
        };

        Ok(Instance { instance })
        */
        unimplemented!();
    }

}
