use crate::AppError;

use ash_urn::descriptor;
use ash_urn::Base;
use ash_urn::DeviceBuffer;
use ash_urn::{Descriptor, DescriptorSettings};

use std::collections::HashMap;

pub fn setup(base: &Base, uniform_buffers: &[DeviceBuffer]) -> Result<Descriptor, AppError> {
    let mut setup_map = HashMap::new();
    setup_map.insert(
        0,
        descriptor::Setup {
            ty: ash::vk::DescriptorType::UNIFORM_BUFFER,
            stage: ash::vk::ShaderStageFlags::VERTEX,
        },
    );
    let mut set_usages = Vec::new();
    for (i, uniform_buffer) in uniform_buffers.iter().enumerate() {
        let mut usages = HashMap::new();
        usages.insert(0, descriptor::Usage::Buffer(uniform_buffer.buffer.0));
        set_usages.push(descriptor::SetUsage {
            usages,
            name: format!("DescriptorSet_{}", i),
        });
    }
    let descriptor = Descriptor::new(
        &base,
        &DescriptorSettings {
            setup_map,
            set_usages,
            name: "Descriptor".to_string(),
        },
    )?;

    Ok(descriptor)
}
