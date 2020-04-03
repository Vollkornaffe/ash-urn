use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Pool(pub ash::vk::DescriptorPool);

impl Pool {
    pub fn new(
        base: &Base,
        pool_sizes: &[ash::vk::DescriptorPoolSize],
        max_sets: u32,
        name: String,
    ) -> Result<Self, UrnError> {
        let pool_info = ash::vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(max_sets);

        let pool = unsafe {
            base.logical_device
                .0
                .create_descriptor_pool(&pool_info, None)?
        };

        base.name_object(pool, name)?;

        Ok(Self(pool))
    }
}
