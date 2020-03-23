pub mod entry;
pub mod instance;
pub mod logical_device;
pub mod physical_device;
pub mod queue_families;
pub mod swapchain;
pub mod validation;

pub use entry::Entry;
pub use instance::{Instance, InstanceSettings};
pub use logical_device::{LogicalDevice, LogicalDeviceSettings};
pub use physical_device::{PhysicalDevice, PhysicalDeviceSettings};
pub use queue_families::{QueueFamily, QueueFamilyKey};
pub use swapchain::SwapChainSupportDetail;
pub use validation::Validation;

/// Very basic setup for a vulkan app.
pub struct Base {
    pub entry: Entry,
    pub instance: Instance,
    pub validation: Option<Validation>,
    pub physical_device: PhysicalDevice,
    pub logical_device: LogicalDevice,
}
