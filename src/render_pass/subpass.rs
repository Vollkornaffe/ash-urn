pub fn dependency() -> ash::vk::SubpassDependencyBuilder<'static> {
    ash::vk::SubpassDependency::builder()
        .src_subpass(ash::vk::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(ash::vk::AccessFlags::default())
        .dst_stage_mask(ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(
            ash::vk::AccessFlags::COLOR_ATTACHMENT_READ
                | ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        )
}
