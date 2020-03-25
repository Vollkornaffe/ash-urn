pub struct SurfaceFormat(pub ash::vk::SurfaceFormatKHR);

impl SurfaceFormat {
    pub fn choose(
        available_formats: &[ash::vk::SurfaceFormatKHR],
    ) -> Self {
        for &format in available_formats {
            if format.format == ash::vk::Format::B8G8R8_UNORM
                && format.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return Self(format);
            }
        }

        Self(available_formats
            .first()
            .unwrap()
            .clone())
    }
}
