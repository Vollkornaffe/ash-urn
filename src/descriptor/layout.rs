use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct SetLayout(pub ash::vk::DescriptorSetLayout);

pub struct SetLayoutSettings<'a> {
    bindings: &'a [ash::vk::DescriptorSetLayoutBinding],
    name: String,
}

impl SetLayout {
    pub fn new(base: &Base, settings: &SetLayoutSettings) -> Result<Self, UrnError> {
        let layout_info =
            ash::vk::DescriptorSetLayoutCreateInfo::builder().bindings(&settings.bindings);
        let layout = unsafe {
            base.logical_device
                .0
                .create_descriptor_set_layout(&layout_info, None)?
        };
        base.name_object(layout, settings.name.clone())?;
        Ok(Self(layout))
    }

    pub fn graphics(base: &Base, name: String) -> Result<Self, UrnError> {
        Self::new(
            base,
            &SetLayoutSettings {
                bindings: &[
                    ash::vk::DescriptorSetLayoutBinding::builder()
                        .binding(0)
                        .descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
                        .descriptor_count(1)
                        .stage_flags(ash::vk::ShaderStageFlags::VERTEX)
                        .build(),
                    ash::vk::DescriptorSetLayoutBinding::builder()
                        .binding(1)
                        .descriptor_type(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                        .descriptor_count(1)
                        .stage_flags(ash::vk::ShaderStageFlags::FRAGMENT)
                        .build(),
                ],
                name,
            },
        )
    }
}
