pub fn attachment_info() -> ash::vk::PipelineColorBlendAttachmentStateBuilder<'static> {
    ash::vk::PipelineColorBlendAttachmentState::builder()
        .color_write_mask(
            ash::vk::ColorComponentFlags::R
                | ash::vk::ColorComponentFlags::G
                | ash::vk::ColorComponentFlags::B
                | ash::vk::ColorComponentFlags::A,
        )
        .blend_enable(false)
        .src_color_blend_factor(ash::vk::BlendFactor::ONE)
        .dst_color_blend_factor(ash::vk::BlendFactor::ZERO)
        .color_blend_op(ash::vk::BlendOp::ADD)
        .src_alpha_blend_factor(ash::vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
        .alpha_blend_op(ash::vk::BlendOp::ADD)
}

pub fn state_info(
    attachment_info: &[ash::vk::PipelineColorBlendAttachmentState],
) -> ash::vk::PipelineColorBlendStateCreateInfoBuilder {
    ash::vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(ash::vk::LogicOp::COPY)
        .attachments(&attachment_info)
        .blend_constants([0.0, 0.0, 0.0, 0.0])
}
