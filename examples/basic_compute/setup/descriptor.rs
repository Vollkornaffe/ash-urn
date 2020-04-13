use crate::AppError;

use ash_urn::descriptor;
use ash_urn::Base;
use ash_urn::DeviceBuffer;
use ash_urn::{Descriptor, DescriptorSettings};
use ash_urn::{DeviceImage, Sampler};

use std::collections::HashMap;

pub fn setup_graphics(
    base: &Base,
    uniform_buffers: &[DeviceBuffer],
    texture: &(DeviceImage, Sampler),
) -> Result<Descriptor, AppError> {
    let mut setup_map = HashMap::new();
    setup_map.insert(
        0,
        descriptor::Setup {
            ty: ash::vk::DescriptorType::UNIFORM_BUFFER,
            stage: ash::vk::ShaderStageFlags::VERTEX,
        },
    );
    setup_map.insert(
        1,
        descriptor::Setup {
            ty: ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            stage: ash::vk::ShaderStageFlags::FRAGMENT,
        },
    );

    let mut set_usages = Vec::new();
    for (i, uniform_buffer) in uniform_buffers.iter().enumerate() {
        let mut usages = HashMap::new();
        usages.insert(0, descriptor::Usage::Buffer(uniform_buffer.buffer.0));
        usages.insert(
            1,
            descriptor::Usage::ImageSampler(
                ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                texture.0.view.0,
                (texture.1).0,
            ),
        );
        set_usages.push(descriptor::SetUsage {
            usages,
            name: format!("GraphicsDescriptorSet_{}", i),
        });
    }
    let descriptor = Descriptor::new(
        &base,
        &DescriptorSettings {
            setup_map,
            set_usages,
            name: "GraphicsDescriptor".to_string(),
        },
    )?;

    Ok(descriptor)
}

pub fn setup_compute(
    base: &Base,
    uniform_buffer: &DeviceBuffer,
    vertex_device_buffer: &DeviceBuffer,
) -> Result<Descriptor, AppError> {
    let mut setup_map = HashMap::new();
    setup_map.insert(
        0,
        descriptor::Setup {
            ty: ash::vk::DescriptorType::UNIFORM_BUFFER,
            stage: ash::vk::ShaderStageFlags::COMPUTE,
        },
    );
    setup_map.insert(
        1,
        descriptor::Setup {
            ty: ash::vk::DescriptorType::STORAGE_BUFFER,
            stage: ash::vk::ShaderStageFlags::COMPUTE,
        },
    );

    let mut set_usages = Vec::new();

    let mut usages = HashMap::new();
    usages.insert(0, descriptor::Usage::Buffer(uniform_buffer.buffer.0));
    usages.insert(1, descriptor::Usage::Buffer(vertex_device_buffer.buffer.0));
    set_usages.push(descriptor::SetUsage {
        usages,
        name: "ComputeDescriptorSet".to_string(),
    });

    let descriptor = Descriptor::new(
        &base,
        &DescriptorSettings {
            setup_map,
            set_usages,
            name: "ComputeDescriptor".to_string(),
        },
    )?;

    Ok(descriptor)
}
