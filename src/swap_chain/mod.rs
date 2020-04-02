use crate::UrnError;
use crate::Base;
use crate::base::SwapChainSupportDetail;
use crate::device_image::{View, ViewSettings};

pub mod extent;
pub mod loader;
pub mod present_mode;
pub mod surface_format;

pub use extent::Extent;
pub use loader::Loader;
pub use present_mode::PresentMode;
pub use surface_format::SurfaceFormat;

use ash::version::DeviceV1_0;

pub struct SwapElement {
    pub image: ash::vk::Image,
    pub image_view: ash::vk::ImageView,
    pub frame_buffer: ash::vk::Framebuffer,
}

pub struct SwapChain {
    pub surface_format: SurfaceFormat,
    pub extent: Extent,
    pub present_mode: PresentMode,
    pub loader: Loader,
    pub handle: ash::vk::SwapchainKHR,
    pub image_count: u32,
    pub elements: Vec<SwapElement>,
    name: String,
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
    pub fn new(base: &Base, settings: &SwapChainSettings) -> Result<Self, UrnError> {
        let surface_format = SurfaceFormat::choose(&settings.support.formats);
        let extent = Extent::choose(settings.w, settings.h, settings.support.capabilities);
        let present_mode = PresentMode::choose(&settings.support.present_modes);
        let loader = Loader::new(base);

        let image_count = if settings.support.capabilities.max_image_count > 0 {
            settings
                .image_count
                .min(settings.support.capabilities.max_image_count)
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

        let handle = unsafe { loader.0.create_swapchain(&swap_chain_create_info, None)? };
        base.name_object(handle, settings.name.clone())?;

        Ok(Self {
            surface_format,
            extent,
            present_mode,
            loader,
            handle,
            image_count,
            elements: Vec::new(),
            name: settings.name.clone(),
        })
    }

    pub fn fill_elements(
        &mut self,
        base: &Base,
        depth_image_view: ash::vk::ImageView,
        render_pass: ash::vk::RenderPass,
    ) -> Result<(), UrnError> {

        let images = unsafe {
            self.loader.0
                .get_swapchain_images(self.handle)?
        };
        for (i, image) in images.iter().enumerate() {
            base.name_object(*image, format!("{}Image_{}", self.name.clone(), i))?;
        }
        
        for i in 0..self.image_count as usize {
            let image_view = View::new(
                base,
                &ViewSettings {
                    image: images[i],
                    format: self.surface_format.0.format,
                    aspect_flags: ash::vk::ImageAspectFlags::COLOR,
                    name: format!("{}ImageView_{}", self.name.clone(), i),
                },
            )?.0;

            let attachments = [image_view, depth_image_view];
            let frame_buffer_info = ash::vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(self.extent.0.width)
                .height(self.extent.0.height)
                .layers(1);
            let frame_buffer = unsafe {
                base.logical_device.0
                    .create_framebuffer(&frame_buffer_info, None)?
            };
            base.name_object(frame_buffer, format!("{}FrameBuffer_{}", self.name.clone(), i))?;

            self.elements.push(
                SwapElement {
                    image: images[i],
                    image_view,
                    frame_buffer,
                }
            );
        }

        Ok(())
    }
}
