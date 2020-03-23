#[derive(PartialEq, Eq, Hash)]
pub struct QueueFamilyKey {
    pub graphics: bool,
    pub present: bool,
    pub transfer: bool,
    pub compute: bool,
}
pub struct QueueFamily {
    pub idx: u32,
    pub properties: ash::vk::QueueFamilyProperties,
}
