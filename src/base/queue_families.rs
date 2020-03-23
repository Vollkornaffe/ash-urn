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
