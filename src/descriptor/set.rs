use crate::UrnError;
use crate::Base;

use super::Setup;
use super::Usage;
use super::SetUsage;

use std::collections::HashMap;

use ash::version::DeviceV1_0;

struct Info {
    binding: u32,
    buffer_infos: Vec<ash::vk::DescriptorBufferInfo>,
    image_infos: Vec<ash::vk::DescriptorImageInfo>,
}

impl Info {

    fn buffer_info(
        binding: u32,
        buffer: ash::vk::Buffer,
    ) -> Self {
        Self {
            binding,
            buffer_infos: vec![
                ash::vk::DescriptorBufferInfo::builder()
                    .buffer(buffer)
                    .offset(0)
                    .range(ash::vk::WHOLE_SIZE)
                    .build()
            ],
            image_infos: vec![],
        }
    }

    fn image_info(
        binding: u32,
        image_layout: ash::vk::ImageLayout,
        image_view: ash::vk::ImageView,
        sampler: ash::vk::Sampler,
    ) -> Self {
        Self {
            binding,
            buffer_infos: vec![],
            image_infos: vec![
                ash::vk::DescriptorImageInfo::builder()
                    .image_layout(image_layout)
                    .image_view(image_view)
                    .sampler(sampler)
                    .build()
            ],
        }
    }
}

pub struct Set(pub ash::vk::DescriptorSet);

impl Set {

    pub fn new(
        base: &Base,
        layout: ash::vk::DescriptorSetLayout,
        pool: ash::vk::DescriptorPool,
        setup_map: &HashMap<u32, Setup>,
        set_usage: &SetUsage,
    ) -> Result<Self, UrnError> {

        // allocation
        let layouts = [layout];
        let alloc_info = ash::vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(pool)
            .set_layouts(&layouts);
        let set = unsafe {
            base.logical_device.0.allocate_descriptor_sets(&alloc_info)? 
        }[0];
        base.name_object(set, set_usage.name.clone())?;

        // prepare infos
        let infos: Vec<Info> = set_usage
            .usages
            .iter()
            .map(|(binding, usage)|
                 match usage {
                    Usage::Buffer(buffer) => Info::buffer_info(*binding, *buffer),
                    Usage::ImageSampler(
                        image_layout,
                        image_view,
                        sampler,
                    ) => Info::image_info(
                        *binding,
                        *image_layout,
                        *image_view,
                        *sampler,
                    ),
                 }
            )
            .collect();

        // create writes from the infos
        let writes: Vec<ash::vk::WriteDescriptorSet> = infos
            .iter()
            .map(|info|
                ash::vk::WriteDescriptorSet::builder()
                    .dst_set(set)
                    .dst_binding(info.binding)
                    .dst_array_element(0)
                    .descriptor_type(setup_map
                        .get(&info.binding)
                        .expect("Invalid binding for descriptor.")
                        .ty
                    )
                    .buffer_info(info.buffer_infos.as_slice())
                    .image_info(info.image_infos.as_slice())
                    .build()
            )
            .collect();

        // no copying supported
        let copies = [];

        // update
        unsafe {
            base.logical_device.0
                .update_descriptor_sets(
                    writes.as_slice(),
                    &copies,
                )
        };

        Ok(Self(set))
    }
}

