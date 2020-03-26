pub fn info() -> ash::vk::PipelineInputAssemblyStateCreateInfoBuilder<'static> {
    ash::vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(ash::vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false)
}
