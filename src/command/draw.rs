use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct DrawIndexedSettings {
    command_buffer: ash::vk::CommandBuffer,
    render_pass: ash::vk::RenderPass,
    frame_buffer: ash::vk::Framebuffer,
    extent: ash::vk::Extent2D,
    graphics_pipeline: ash::vk::Pipeline,
    graphics_pipeline_layout: ash::vk::PipelineLayout,
    descriptor_set: ash::vk::DescriptorSet,
    vertex_buffer: ash::vk::Buffer,
    index_buffer: ash::vk::Buffer,
    n_indices: u32,
}

pub fn indexed(
    base: &Base,
    settings: &DrawIndexedSettings,
) -> Result<(), UrnError> {

    let clear_values = [
        ash::vk::ClearValue {
            color: ash::vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        },
        ash::vk::ClearValue {
            depth_stencil: ash::vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            },
        },
    ];

    let render_pass_info = ash::vk::RenderPassBeginInfo::builder()
        .render_pass(settings.render_pass)
        .framebuffer(settings.frame_buffer)
        .render_area(ash::vk::Rect2D {
            offset: ash::vk::Offset2D { x: 0, y: 0 },
            extent: settings.extent,
        })
        .clear_values(&clear_values);

    let vertex_buffers = [settings.vertex_buffer];
    let offsets = [0];
    let dynamic_offsets = [];
    let descriptor_sets = [settings.descriptor_set];
    unsafe {
        base.logical_device.0.cmd_begin_render_pass(
            settings.command_buffer,
            &render_pass_info,
            ash::vk::SubpassContents::INLINE,
        );
        base.logical_device.0.cmd_bind_pipeline(
            settings.command_buffer,
            ash::vk::PipelineBindPoint::GRAPHICS,
            settings.graphics_pipeline,
        );
        base.logical_device.0.cmd_bind_vertex_buffers(
            settings.command_buffer,
            0,
            &vertex_buffers,
            &offsets,
        );
        base.logical_device.0.cmd_bind_index_buffer(
            settings.command_buffer,
            settings.index_buffer,
            0,
            ash::vk::IndexType::UINT32,
        );
        base.logical_device.0.cmd_bind_descriptor_sets(
            settings.command_buffer,
            ash::vk::PipelineBindPoint::GRAPHICS,
            settings.graphics_pipeline_layout,
            0,
            &descriptor_sets,
            &dynamic_offsets,
        );
        base.logical_device.0
            .cmd_draw_indexed(settings.command_buffer, settings.n_indices, 1, 0, 0, 0);
        base.logical_device.0.cmd_end_render_pass(settings.command_buffer);
        base.logical_device.0.end_command_buffer(settings.command_buffer)?;
    }

    Ok(())
}
