use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Pool(pub ash::vk::DescriptorPool);

pub struct PoolSettings<'a> {
    pool_sizes: &'a [ash::vk::DescriptorPoolSize],
    max_sets: u32,
    name: String,
}

impl Pool {
    pub fn new(
        base: &Base,
        settings: &PoolSettings,
    ) -> Result<Self, UrnError> {
        let pool_info = ash::vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&settings.pool_sizes)
            .max_sets(settings.max_sets);
        let pool = unsafe { base.logical_device.0.create_descriptor_pool(&pool_info, None)? };
        base.name_object(pool, "GraphicsDescriptorPool".to_string())?;
        Ok(Self(pool))
    }

    pub fn graphics(
        base: &Base,
        image_count: u32,
        name: String,
    ) -> Result<Self, UrnError> {
        Self::new(
            base,
            &PoolSettings {
                pool_sizes: &[
                    ash::vk::DescriptorPoolSize::builder()
                        .ty(ash::vk::DescriptorType::UNIFORM_BUFFER)
                        .descriptor_count(image_count)
                        .build(),
                    ash::vk::DescriptorPoolSize::builder()
                        .ty(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                        .descriptor_count(image_count)
                        .build(),
                ],
                max_sets: image_count,
                name: name,
            },
        )
    }
}
