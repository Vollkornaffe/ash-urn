use crate::Base;
use crate::UrnError;

use crate::pipeline::{ShaderModule, ShaderModuleSettings};

use ash::version::DeviceV1_0;

pub struct ComputePipeline(pub ash::vk::Pipeline);

pub struct ComputePipelineSettings {
    pub layout: ash::vk::PipelineLayout,
    pub comp_spv: String,
    pub name: String,
}

impl ComputePipeline {
    pub fn new(base: &Base, settings: &ComputePipelineSettings) -> Result<Self, UrnError> {
        let shader_name = std::ffi::CString::new("main").unwrap();
        let shader_module_comp = ShaderModule::new(
            &base,
            &ShaderModuleSettings {
                file_name: settings.comp_spv.clone(),
                name: format!("{}Shader", settings.name.clone()),
            },
        )?;
        let comp_stage_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .stage(ash::vk::ShaderStageFlags::COMPUTE)
            .module(shader_module_comp.0)
            .name(&shader_name);
        let pipeline_info = ash::vk::ComputePipelineCreateInfo::builder()
            .stage(comp_stage_info.build())
            .layout(settings.layout);

        let pipeline_infos = [pipeline_info.build()];
        let pipeline = unsafe {
            base.logical_device.0.create_compute_pipelines(
                ash::vk::PipelineCache::default(),
                &pipeline_infos,
                None,
            )
        }
        .map_err(|(_, e)| e)?[0];
        base.name_object(pipeline, settings.name.clone())?;

        unsafe {
            base.logical_device
                .0
                .destroy_shader_module(shader_module_comp.0, None);
        }

        Ok(Self(pipeline))
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_pipeline(self.0, None);
        }
    }
}
