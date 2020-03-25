use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Layout(pub ash::vk::PipelineLayout);

pub struct LayoutSettings {
    set_layouts: Vec<ash::vk::DescriptorSetLayout>,
    push_constant_ranges: Vec<ash::vk::PushConstantRange>,
    name: String,
}

impl Layout {
    pub fn new(
        base: &Base,
        settings: &LayoutSettings,
    ) -> Result<Self, UrnError> {

        let pipeline_layout_info = ash::vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(&settings.set_layouts)
            .push_constant_ranges(&settings.push_constant_ranges);

        let layout = unsafe {
            base.logical_device.0
                .create_pipeline_layout(&pipeline_layout_info, None)?
        };
        base.name_object(layout, settings.name.clone())?;

        Ok(Self(layout))
    }
}
