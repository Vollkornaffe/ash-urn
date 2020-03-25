pub fn info(extent: ash::vk::Extent2D) -> ash::vk::ViewportBuilder<'static> {
    ash::vk::Viewport::builder()
        .x(0.0)
        .y(0.0)
        .width(extent.width as f32)
        .height(extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)
}
