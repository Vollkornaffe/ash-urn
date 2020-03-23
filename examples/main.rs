mod sdl;

use ash_urn::base::{
    Base, Entry, Instance, InstanceSettings, LogicalDevice, PhysicalDevice, SwapChainSupportDetail,
    Validation,
};

const ENABLE_VALIDATION: bool = cfg!(debug_assertions);

fn main() {
    let mut sdl = sdl::SDL::new(sdl::WindowSettings {
        title: "Test",
        w: 800,
        h: 800,
        maximized: false,
    })
    .unwrap();

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
    let instance = Instance::new(
        InstanceSettings {
            name: "Test".to_string(),
            version_major: 1,
            version_minor: 2,
            version_patch: 131,
            extension_names: instance_extension_names,
            enable_validation: ENABLE_VALIDATION,
            validation_layer_names: validation_layer_names,
        },
        &entry.0,
    )
    .unwrap();
    let validation = if ENABLE_VALIDATION {
        Some(Validation::new(&entry.0, &instance.0).unwrap())
    } else {
        None
    };
    let surface_loader = ash::extensions::khr::Surface::new(&entry.0, &instance.0);
    let surface = sdl.create_surface(&instance.0).unwrap();
    let device_extensions = vec![
        "VK_KHR_swapchain".to_string(),
        "VK_KHR_timeline_semaphore".to_string(),
    ];
    let physical_device = PhysicalDevice::pick_gpu(
        &instance.0,
        device_extensions.clone(),
        &surface_loader,
        surface,
    )
    .unwrap();


    'running: loop {
        for e in sdl.get_events() {
            match e {
                sdl::SdlEvent::Close => break 'running,
                _ => {}
            }
        }
    }
}
