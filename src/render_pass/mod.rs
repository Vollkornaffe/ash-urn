use crate::UrnError;
use crate::Base;

mod attachment;
mod subpass;

use ash::version::DeviceV1_0;

pub struct RenderPass(pub ash::vk::RenderPass);

pub struct RenderPassSettings {
    pub depth: bool,
    pub swap_chain_format: ash::vk::Format,
    pub name: String,
}

impl RenderPass {
    pub fn new(
        base: &Base,
        settings: &RenderPassSettings,
    ) -> Result<Self, UrnError> {
        
        let mut attachment_descriptions = vec![
            attachment::color_description(settings.swap_chain_format),
        ];
        if settings.depth {
            attachment_descriptions.push(attachment::depth_description(base)?);
        }

        let color_attachment_refs = [ash::vk::AttachmentReference::builder()
            .attachment(0)
            .layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()
        ];
        let depth_attachment_ref = ash::vk::AttachmentReference::builder()
            .attachment(1)
            .layout(ash::vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let subpass_description = ash::vk::SubpassDescription::builder()
            .pipeline_bind_point(ash::vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_refs);
        let subpass_description = if settings.depth {
            subpass_description.depth_stencil_attachment(&depth_attachment_ref)
        } else {
            subpass_description
        };

        let subpass_dependency = subpass::dependency();

        let subpass_descriptions = [subpass_description.build()];
        let subpass_dependencies = [subpass_dependency];
        let render_pass_info = ash::vk::RenderPassCreateInfo::builder()
            .attachments(&attachment_descriptions)
            .subpasses(&subpass_descriptions)
            .dependencies(&subpass_dependencies);

        let render_pass = unsafe {
            base.logical_device.0
                .create_render_pass(&render_pass_info, None)?
        };
        base.name_object(render_pass, settings.name.clone())?;

        Ok(Self(render_pass))
    }
}