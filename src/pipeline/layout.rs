use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct PipelineLayout(pub ash::vk::PipelineLayout);

pub struct PipelineLayoutSettings {
    pub set_layouts: Vec<ash::vk::DescriptorSetLayout>,
    pub push_constant_ranges: Vec<ash::vk::PushConstantRange>,
    pub name: String,
}

impl PipelineLayout {
    pub fn new(base: &Base, settings: &PipelineLayoutSettings) -> Result<Self, UrnError> {
        let pipeline_layout_info = ash::vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(&settings.set_layouts)
            .push_constant_ranges(&settings.push_constant_ranges);

        let layout = unsafe {
            base.logical_device
                .0
                .create_pipeline_layout(&pipeline_layout_info, None)?
        };
        base.name_object(layout, settings.name.clone())?;

        Ok(Self(layout))
    }
}
