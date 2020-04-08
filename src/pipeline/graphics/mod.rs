use crate::mesh::Vertex;
use crate::Base;
use crate::UrnError;

use crate::pipeline::{ShaderModule, ShaderModuleSettings};

use ash::version::DeviceV1_0;

mod color_blend;
mod depth_stencil;
mod input_assembly;
mod multisampling;
mod rasterizer;
mod scissor;
mod viewport;

pub struct GraphicsPipeline(pub ash::vk::Pipeline);

pub struct GraphicsPipelineSettings {
    pub layout: ash::vk::PipelineLayout,
    pub vert_spv: String,
    pub frag_spv: String,
    pub extent: ash::vk::Extent2D,
    pub render_pass: ash::vk::RenderPass,
    pub name: String,
}

impl GraphicsPipeline {
    pub fn new(base: &Base, settings: &GraphicsPipelineSettings) -> Result<Self, UrnError> {
        let shader_name = std::ffi::CString::new("main").unwrap();

        let shader_module_vert = ShaderModule::new(
            &base,
            &ShaderModuleSettings {
                file_name: settings.vert_spv.clone(),
                name: "VertexShader".to_string(),
            },
        )?;
        let shader_module_frag = ShaderModule::new(
            &base,
            &ShaderModuleSettings {
                file_name: settings.frag_spv.clone(),
                name: "FragmentShader".to_string(),
            },
        )?;

        let vert_stage_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .stage(ash::vk::ShaderStageFlags::VERTEX)
            .module(shader_module_vert.0)
            .name(&shader_name);
        let frag_stage_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .stage(ash::vk::ShaderStageFlags::FRAGMENT)
            .module(shader_module_frag.0)
            .name(&shader_name);
        let shader_stage_infos = [vert_stage_info.build(), frag_stage_info.build()];

        let vertex_binding = Vertex::get_binding_description();
        let vertex_attributes = Vertex::get_attribute_description();
        let vertex_input_info = ash::vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&vertex_binding)
            .vertex_attribute_descriptions(&vertex_attributes);

        let input_assembly_info = input_assembly::info();

        let viewports = [viewport::info(settings.extent).build()];
        let scissors = [scissor::info(settings.extent).build()];
        let viewport_state = ash::vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer_info = rasterizer::info();
        let multisampling_info = multisampling::info();

        let color_blend_attachments = [color_blend::attachment_info().build()];
        let color_blend_state_info = color_blend::state_info(&color_blend_attachments);

        let depth_stencil_info = depth_stencil::info();

        let pipeline_info = ash::vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage_infos)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly_info)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisampling_info)
            .color_blend_state(&color_blend_state_info)
            .depth_stencil_state(&depth_stencil_info)
            .layout(settings.layout)
            .render_pass(settings.render_pass)
            .subpass(0);

        let pipeline_infos = [pipeline_info.build()];

        let pipeline = unsafe {
            base.logical_device.0.create_graphics_pipelines(
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
                .destroy_shader_module(shader_module_vert.0, None);
            base.logical_device
                .0
                .destroy_shader_module(shader_module_frag.0, None);
        }

        Ok(Self(pipeline))
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_pipeline(self.0, None);
        }
    }
}
