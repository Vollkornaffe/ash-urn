use crate::error::UrnError;

#[derive(PartialEq, Eq, Hash)]
pub struct QueueFamilyKey {
    pub graphics: bool,
    pub present: bool,
    pub transfer: bool,
    pub compute: bool,
}
pub const COMBINED: QueueFamilyKey = QueueFamilyKey {
    graphics: true,
    present: true,
    transfer: true,
    compute: true,
};
pub const DEDICATED_TRANSFER: QueueFamilyKey = QueueFamilyKey { 
    graphics: false,
    present: false,
    transfer: true,
    compute: false,
};
pub struct QueueFamily {
    pub idx: u32,
    pub properties: ash::vk::QueueFamilyProperties,
}
impl QueueFamilyKey {
    pub fn gen_key(
        queue_family: &QueueFamily,
        physical_device: ash::vk::PhysicalDevice,
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<Self, UrnError> {
        let flags = queue_family.properties.queue_flags;
        Ok(Self {
            graphics: flags.contains(ash::vk::QueueFlags::GRAPHICS),
            present: unsafe {
                surface_loader
                    .get_physical_device_surface_support(physical_device, queue_family.idx, surface)?
            },
            transfer: flags.contains(ash::vk::QueueFlags::TRANSFER),
            compute: flags.contains(ash::vk::QueueFlags::COMPUTE),
        })
    }
}

