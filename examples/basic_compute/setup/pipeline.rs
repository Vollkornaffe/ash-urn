use crate::AppError;

use ash_urn::Base;
use ash_urn::Descriptor;
use ash_urn::RenderPass;
use ash_urn::SwapChain;
use ash_urn::UrnVertex;
use ash_urn::{ComputePipeline, ComputePipelineSettings};
use ash_urn::{GraphicsPipeline, GraphicsPipelineSettings};
use ash_urn::{PipelineLayout, PipelineLayoutSettings};

pub fn setup_graphics(
    base: &Base,
    descriptor: &Descriptor,
    swap_chain: &SwapChain,
    render_pass: &RenderPass,
) -> Result<(PipelineLayout, GraphicsPipeline), AppError> {
    let pipeline_layout = PipelineLayout::new(
        &base,
        &PipelineLayoutSettings {
            set_layouts: vec![descriptor.layout.0],
            push_constant_ranges: vec![],
            name: "GraphicsPipelineLayout".to_string(),
        },
    )?;

    let pipeline = GraphicsPipeline::new::<UrnVertex>(
        &base,
        &GraphicsPipelineSettings {
            layout: pipeline_layout.0,
            vert_spv: &std::path::Path::new("examples/basic_compute/shaders/vert.spv"),
            frag_spv: &std::path::Path::new("examples/basic_compute/shaders/frag.spv"),
            extent: swap_chain.extent.0,
            render_pass: render_pass.0,
            name: "GraphicsPipeline".to_string(),
        },
    )?;

    Ok((pipeline_layout, pipeline))
}

pub fn setup_compute(
    base: &Base,
    descriptor: &Descriptor,
) -> Result<(PipelineLayout, ComputePipeline, ComputePipeline), AppError> {
    let pipeline_layout = PipelineLayout::new(
        &base,
        &PipelineLayoutSettings {
            set_layouts: vec![descriptor.layout.0],
            push_constant_ranges: vec![],
            name: "ComputePipelineLayout".to_string(),
        },
    )?;

    let calculate_pipeline = ComputePipeline::new(
        &base,
        &ComputePipelineSettings {
            layout: pipeline_layout.0,
            comp_spv: &std::path::Path::new("examples/basic_compute/shaders/calculate.spv"),
            name: "CalculatePipeline".to_string(),
        },
    )?;

    let integrate_pipeline = ComputePipeline::new(
        &base,
        &ComputePipelineSettings {
            layout: pipeline_layout.0,
            comp_spv: &std::path::Path::new("examples/basic_compute/shaders/integrate.spv"),
            name: "IntegratePipeline".to_string(),
        },
    )?;

    Ok((pipeline_layout, calculate_pipeline, integrate_pipeline))
}
