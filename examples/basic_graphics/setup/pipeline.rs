use crate::AppError;

use ash_urn::Base;
use ash_urn::Descriptor;
use ash_urn::RenderPass;
use ash_urn::SwapChain;
use ash_urn::UrnVertex;
use ash_urn::{GraphicsPipeline, GraphicsPipelineSettings};
use ash_urn::{PipelineLayout, PipelineLayoutSettings};

pub fn setup(
    base: &Base,
    descriptor: &Descriptor,
    swap_chain: &SwapChain,
    render_pass: &RenderPass,
) -> Result<(PipelineLayout, GraphicsPipeline), AppError> {
    let graphics_pipeline_layout = PipelineLayout::new(
        &base,
        &PipelineLayoutSettings {
            set_layouts: vec![descriptor.layout.0],
            push_constant_ranges: vec![],
            name: "GraphicsPipelineLayout".to_string(),
        },
    )?;

    let graphics_pipeline = GraphicsPipeline::new::<UrnVertex>(
        &base,
        &GraphicsPipelineSettings {
            layout: graphics_pipeline_layout.0,
            vert_spv: &std::path::Path::new("examples/basic_graphics/shaders/vert.spv"),
            frag_spv: &std::path::Path::new("examples/basic_graphics/shaders/frag.spv"),
            extent: swap_chain.extent.0,
            render_pass: render_pass.0,
            name: "GraphicsPipeline".to_string(),
        },
    )?;

    Ok((graphics_pipeline_layout, graphics_pipeline))
}
