use crate::AppError;
use crate::SDL;

use ash_urn::device_image::create_depth_device_image;
use ash_urn::Base;
use ash_urn::DeviceImage;
use ash_urn::{RenderPass, RenderPassSettings};
use ash_urn::{SwapChain, SwapChainSettings};

pub fn setup(
    base: &Base,
    sdl: &SDL,
    surface_loader: &ash::extensions::khr::Surface,
    surface: ash::vk::SurfaceKHR,
) -> Result<(SwapChain, RenderPass, DeviceImage), AppError> {
    let swap_chain_support = base
        .physical_device
        .query_swap_chain_support(&surface_loader, surface)
        .unwrap();
    let (w,h) = sdl.get_size();
    let mut swap_chain = SwapChain::new(
        &base,
        &SwapChainSettings {
            w,
            h,
            support: swap_chain_support,
            surface: surface,
            image_count: 3,
            name: "SwapChain".to_string(),
        },
    )?;

    let render_pass = RenderPass::new(
        &base,
        &RenderPassSettings {
            swap_chain_format: swap_chain.surface_format.0.format,
            name: "RenderPass".to_string(),
        },
    )?;

    let depth_device_image = create_depth_device_image(&base, swap_chain.extent.0).unwrap();

    // now we can fill out the swapchain elements
    swap_chain
        .fill_elements(&base, depth_device_image.view.0, render_pass.0)
        .unwrap();

    Ok((swap_chain, render_pass, depth_device_image))
}
