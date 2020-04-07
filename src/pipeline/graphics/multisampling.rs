pub fn info() -> ash::vk::PipelineMultisampleStateCreateInfoBuilder<'static> {
    ash::vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
}
