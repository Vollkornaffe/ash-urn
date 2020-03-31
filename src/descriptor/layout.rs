use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Layout(pub ash::vk::DescriptorSetLayout);

impl Layout {
    pub fn new(
        base: &Base,
        bindings: &[ash::vk::DescriptorSetLayoutBinding],
        name: String,
    ) -> Result<Self, UrnError> {
        let layout_info = ash::vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);
        let layout = unsafe {
            base.logical_device.0
                .create_descriptor_set_layout(&layout_info, None)?
        };
        base.name_object(layout, name)?;

        Ok(Self(layout))
    }
}
