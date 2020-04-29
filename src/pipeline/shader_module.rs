use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct ShaderModule(pub ash::vk::ShaderModule);

pub struct ShaderModuleSettings {
    pub file_name: String,
    pub name: String,
}

impl ShaderModule {
    pub fn new(base: &Base, settings: &ShaderModuleSettings) -> Result<Self, UrnError> {
        let mut f = std::fs::File::open(settings.file_name.clone()).map_err(|_| {
            UrnError::GenericDynamic(format!(
                "Failed to open shader file: {}",
                settings.file_name.clone()
            ))
        })?;
        let buffer = ash::util::read_spv(&mut f)?;

        let create_info = ash::vk::ShaderModuleCreateInfo::builder().code(buffer.as_slice());

        let shader_module = unsafe {
            base.logical_device
                .0
                .create_shader_module(&create_info, None)?
        };
        base.name_object(shader_module, settings.name.clone())?;

        Ok(Self(shader_module))
    }
}
