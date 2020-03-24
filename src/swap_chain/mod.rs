use crate::UrnError;
use crate::Base;

pub mod format;
pub mod extent;
pub mod present_mode;
pub mod loader;

pub use format::Format;
pub use extent::Extent;
pub use present_mode::PresentMode;
pub use loader::Loader;

pub struct SwapChain {
    pub format: Format,
    pub extent: Extent,
    pub present_mode: PresentMode,
    pub loader: Loader,
    pub swap_chain: ash::vk::SwapchainKHR,

    // the images supplied by the chain
    pub images: Vec<ash::vk::Image>,
}

pub struct SwapChainSettings {
    available_formats: Vec<ash::vk::SurfaceFormatKHR>,
    w: u32,
    h: u32,
    capabilities: ash::vk::SurfaceCapabilitiesKHR,
    available_modes: Vec<ash::vk::PresentModeKHR>,
    surface: ash::vk::SurfaceKHR,
    image_count: u32,
    name: String,
}

impl SwapChain {
    pub fn new(
        base: &Base,
        settings: &SwapChainSettings,
    ) -> Result<Self, UrnError> {

        let format = Format::choose(&settings.available_formats);
        let extent = Extent::choose(
            settings.w,
            settings.h,
            settings.capabilities,
        );
        let present_mode = PresentMode::choose(&settings.available_modes);
        let loader = Loader::new(base);

        let image_count = if settings.capabilities.max_image_count > 0 {
            settings.image_count.min(settings.capabilities.max_image_count)
        } else {
            settings.image_count
        };

        let swap_chain_create_info = ash::vk::SwapchainCreateInfoKHR::builder()
            .surface(settings.surface)
            .min_image_count(image_count)
            .image_color_space(format.0.color_space)
            .image_format(format.0.format)
            .image_extent(extent.0)
            .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .pre_transform(settings.capabilities.current_transform)
            .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode.0)
            .clipped(true)
            .image_array_layers(1);

        let swap_chain = unsafe {
            loader.0
                .create_swapchain(&swap_chain_create_info, None)?
        };
        base.name_object(swap_chain, settings.name.clone())?;

        let images = unsafe {
            loader.0
                .get_swapchain_images(swap_chain)?
        };

        Ok(Self {
            format,
            extent,
            present_mode,
            loader,
            swap_chain,
            images,
        })
    }
}
