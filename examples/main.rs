mod sdl;

use ash_urn::base::{Base, Entry, Instance, LogicalDevice, PhysicalDevice, Validation};

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
    instance_extension_names.push(ash::extensions::ext::DebugUtils::name().to_str().unwrap());
    instance_extension_names.push("VK_KHR_get_physical_device_properties2");
    let validation_layer_names = ["VK_LAYER_KHRONOS_validation"];

    let entry = Entry::new().unwrap();
    let instance = Instance::new("Test", 1, 2, 131, &instance_extension_names, 
                                 ENABLE_VALIDATION, &validation_layer_names,
                                 &entry.0).unwrap();
    let validation = if ENABLE_VALIDATION {
        Some(Validation::new(
       &entry.0, &instance.0         
        ).unwrap())
    } else {
        None
    };
    let surface = sdl.create_surface(&instance.0).unwrap();

    'running: loop {
        for e in sdl.get_events() {
            match e {
                sdl::SdlEvent::Close => break 'running,
                _ => {}
            }
        }
    }
}
