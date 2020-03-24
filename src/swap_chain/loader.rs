use crate::Base;

pub struct Loader(pub ash::extensions::khr::Swapchain);

impl Loader {
    pub fn new(
        base: &Base,
    ) -> Self {
        Self(ash::extensions::khr::Swapchain::new(
            &base.instance.0,
            &base.logical_device.0,
        ))
    }
}
