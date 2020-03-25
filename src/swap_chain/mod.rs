use crate::UrnError;
use crate::Base;
use crate::base::SwapChainSupportDetail;

pub mod surface_format;
pub mod extent;
pub mod present_mode;
pub mod loader;

pub use surface_format::SurfaceFormat;
pub use extent::Extent;
pub use present_mode::PresentMode;
pub use loader::Loader;

pub struct SwapChain {
    pub surface_format: SurfaceFormat,
    pub extent: Extent,
    pub present_mode: PresentMode,
    pub loader: Loader,
    pub handle: ash::vk::SwapchainKHR,
}

pub struct SwapChainSettings {
    pub w: u32,
    pub h: u32,
    pub support: SwapChainSupportDetail,
    pub surface: ash::vk::SurfaceKHR,
    pub image_count: u32,
    pub name: String,
}

impl SwapChain {
    pub fn new(
        base: &Base,
        settings: &SwapChainSettings,
    ) -> Result<Self, UrnError> {

        let surface_format = SurfaceFormat::choose(&settings.support.formats);
        let extent = Extent::choose(
            settings.w,
            settings.h,
            settings.support.capabilities,
        );
        let present_mode = PresentMode::choose(&settings.support.present_modes);
        let loader = Loader::new(base);

        let image_count = if settings.support.capabilities.max_image_count > 0 {
            settings.image_count.min(settings.support.capabilities.max_image_count)
        } else {
            settings.image_count
        };

        let swap_chain_create_info = ash::vk::SwapchainCreateInfoKHR::builder()
            .surface(settings.surface)
            .min_image_count(image_count)
            .image_color_space(surface_format.0.color_space)
            .image_format(surface_format.0.format)
            .image_extent(extent.0)
            .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .pre_transform(settings.support.capabilities.current_transform)
            .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode.0)
            .clipped(true)
            .image_array_layers(1);

        let handle = unsafe {
            loader.0
                .create_swapchain(&swap_chain_create_info, None)?
        };
        base.name_object(handle, settings.name.clone())?;

        Ok(Self {
            surface_format,
            extent,
            present_mode,
            loader,
            handle,
        })
    }
}
