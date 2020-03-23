pub mod entry;
pub mod instance;
pub mod logical_device;
pub mod physical_device;
pub mod validation;

pub use entry::Entry;
pub use instance::Instance;
pub use logical_device::LogicalDevice;
pub use physical_device::PhysicalDevice;
pub use validation::Validation;

/// Very basic setup for a vulkan app.
pub struct Base {
    entry: Entry,
    instance: Instance,
    validation: Validation,
    physical_device: PhysicalDevice,
    logical_device: LogicalDevice,
}
