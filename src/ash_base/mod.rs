pub mod entry;
pub mod instance;
pub mod validation;
pub mod physical_device;
pub mod logical_device;

/// Very basic setup for a vulkan app.
pub struct AshBase {
    entry: entry::Entry,
    instance: instance::Instance,
    validation: validation::Validation,
    physical_device: physical_device::PhysicalDevice,
    logical_device: logical_device::LogicalDevice,
}
