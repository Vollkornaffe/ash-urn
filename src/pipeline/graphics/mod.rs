use crate::mesh::Vertex;
use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

mod color_blend;
mod depth_stencil;
mod input_assembly;
mod multisampling;
mod rasterizer;
mod scissor;
mod vertex_input;
mod viewport;

pub struct GraphicsPipeline(pub ash::vk::Pipeline);

pub struct GraphicsPipelineSettings {
    pub layout: ash::vk::PipelineLayout,
    pub vert_module: ash::vk::ShaderModule,
    pub frag_module: ash::vk::ShaderModule,
    pub extent: ash::vk::Extent2D,
    pub render_pass: ash::vk::RenderPass,
    pub name: String,
}

impl GraphicsPipeline {
    pub fn new(base: &Base, settings: &GraphicsPipelineSettings) -> Result<Self, UrnError> {
        let shader_name = std::ffi::CString::new("main").unwrap();

        let vert_stage_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .stage(ash::vk::ShaderStageFlags::VERTEX)
            .module(settings.vert_module)
            .name(&shader_name);
        let frag_stage_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .stage(ash::vk::ShaderStageFlags::FRAGMENT)
            .module(settings.frag_module)
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
        let sample_mask = [];
        let multisampling_info = multisampling::info(&sample_mask);

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

        Ok(Self(pipeline))
    }
}
