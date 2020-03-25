pub fn info(sample_mask: &[ash::vk::SampleMask]) -> ash::vk::PipelineMultisampleStateCreateInfoBuilder {
    ash::vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        .sample_mask(&sample_mask)
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
}
