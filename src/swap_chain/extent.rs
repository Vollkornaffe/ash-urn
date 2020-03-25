pub struct Extent(pub ash::vk::Extent2D);

impl Extent {
    pub fn choose(
        w: u32,
        h: u32,
        capabilities: ash::vk::SurfaceCapabilitiesKHR,
    ) -> Self {
        if capabilities.current_extent.height == std::u32::MAX {
            // The extent of the swapchain can be choosen freely
            Self(capabilities.current_extent)
        } else {
            Self(ash::vk::Extent2D {
                width: std::cmp::max(
                    capabilities.min_image_extent.width,
                    std::cmp::min(capabilities.max_image_extent.width, w),
                ),
                height: std::cmp::max(
                    capabilities.min_image_extent.height,
                    std::cmp::min(capabilities.max_image_extent.height, h),
                ),
            })
        }
    }
}
