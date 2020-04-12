pub fn info() -> ash::vk::PipelineRasterizationStateCreateInfoBuilder<'static> {
    ash::vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(ash::vk::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(ash::vk::CullModeFlags::BACK)
        .front_face(ash::vk::FrontFace::COUNTER_CLOCKWISE)
        .depth_bias_enable(false)
        .depth_bias_constant_factor(0.0)
        .depth_bias_clamp(0.0)
        .depth_bias_slope_factor(0.0)
}
