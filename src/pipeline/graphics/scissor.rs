pub fn info(extent: ash::vk::Extent2D) -> ash::vk::Rect2DBuilder<'static> {
    ash::vk::Rect2D::builder()
        .offset(ash::vk::Offset2D { x: 0, y: 0 })
        .extent(extent)
}
