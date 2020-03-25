mod sdl;

use ash_urn::base::{
    Base, Entry, Instance, InstanceSettings, LogicalDevice, LogicalDeviceSettings, PhysicalDevice,
    PhysicalDeviceSettings, Validation,
};

use ash_urn::{RenderPass, RenderPassSettings};
use ash_urn::{SwapChain, SwapChainSettings};

use ash::version::DeviceV1_0;

const ENABLE_VALIDATION: bool = cfg!(debug_assertions);

fn main() {
    // first of all create sdl context
    let mut sdl = sdl::SDL::new(sdl::WindowSettings {
        title: "Test",
        w: 800,
        h: 800,
        maximized: false,
    })
    .unwrap();

    // Get our requriements ready
    let mut instance_extension_names = sdl.required_extension_names().unwrap();
    instance_extension_names.push(
        ash::extensions::ext::DebugUtils::name()
            .to_str()
            .unwrap()
            .to_string(),
    );
    instance_extension_names.push("VK_KHR_get_physical_device_properties2".to_string());
    let validation_layer_names = vec!["VK_LAYER_KHRONOS_validation".to_string()];

    let entry = Entry::new().unwrap();
    // Instance needs vulkan version
    let instance = Instance::new(
        InstanceSettings {
            name: "Test".to_string(),
            version_major: 1,
            version_minor: 2,
            version_patch: 131,
            extension_names: instance_extension_names,
            enable_validation: ENABLE_VALIDATION,
            validation_layer_names: validation_layer_names.clone(),
        },
        &entry.0,
    )
    .unwrap();
    // Get our input validated!
    let validation = if ENABLE_VALIDATION {
        Some(Validation::new(&entry.0, &instance.0).unwrap())
    } else {
        None
    };
    // Ready for the surface to draw on
    let surface_loader = ash::extensions::khr::Surface::new(&entry.0, &instance.0);
    let surface = sdl.create_surface(&instance.0).unwrap();

    // Time to think about devices
    let timelines = false;
    let mut device_extensions = vec!["VK_KHR_swapchain".to_string()];
    if timelines {
        device_extensions.push("VK_KHR_timeline_semaphore".to_string());
    }
    // First get the actual gpu
    let physical_device = PhysicalDevice::pick_gpu(
        &instance.0,
        device_extensions.clone(),
        &surface_loader,
        surface,
        PhysicalDeviceSettings {
            timelines,
            subgroups: true,
        },
    )
    .unwrap();

    let queue_map = physical_device
        .query_queues(&instance.0, &surface_loader, surface)
        .unwrap();
    let transfer_queue = queue_map
        .get(&ash_urn::base::queue_families::DEDICATED_TRANSFER)
        .unwrap()
        .idx;
    let combined_queue = queue_map
        .get(&ash_urn::base::queue_families::COMBINED)
        .unwrap()
        .idx;

    // Then the logical device that does all of the heavy lifting
    let logical_device = LogicalDevice::new(
        &instance.0,
        physical_device.0,
        LogicalDeviceSettings {
            extension_names: device_extensions,
            enable_validation: ENABLE_VALIDATION,
            validation_layer_names: validation_layer_names.clone(),
            queues: vec![transfer_queue, combined_queue],
            timelines,
        },
    )
    .unwrap();

    // Combine everything into the Base
    let base = Base {
        entry,
        instance,
        validation,
        physical_device,
        logical_device,
    };

    // Create swapchain
    let swap_chain_support = base
        .physical_device
        .query_swap_chain_support(&surface_loader, surface)
        .unwrap();
    let swap_chain = SwapChain::new(
        &base,
        &SwapChainSettings {
            w: sdl.window.size().0,
            h: sdl.window.size().1,
            support: swap_chain_support,
            surface: surface,
            image_count: 2,
            name: "SwapChain".to_string(),
        },
    )
    .unwrap();

    // Create render pass
    let render_pass = RenderPass::new(
        &base,
        &RenderPassSettings {
            depth: false,
            swap_chain_format: swap_chain.surface_format.0.format,
            name: "RenderPass".to_string(),
        },
    )
    .unwrap();

    // Create a single graphics pipeline
    // TODO: need to make descriptors, shader modules and push constants first
    //let graphics_pipeline_layout = pipeline::Layout::new(
    //    &base,
    //    &pipeline::LayoutSettings {
    //
    //    }
    //).unwrap();
    //let graphics_pipeline = pipeline::Graphics::new(
    //    &base,
    //    &pipeline::GraphicsSettings {
    //
    //    },
    //).unwrap();

    'running: loop {
        for e in sdl.get_events() {
            match e {
                sdl::SdlEvent::Close => break 'running,
                _ => {}
            }
        }
    }

    unsafe {
        base.logical_device
            .0
            .destroy_render_pass(render_pass.0, None);
        swap_chain
            .loader
            .0
            .destroy_swapchain(swap_chain.handle, None);
        surface_loader.destroy_surface(surface, None);
    }
}
