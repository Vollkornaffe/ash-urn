use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Pool(pub ash::vk::CommandPool);

/// All pools must be destroyed before the logical_device
impl Pool {
    pub fn new(
        base: &Base,
        queue_family_index: u32,
        name: String,
    ) -> Result<Self, UrnError> {

        let command_pool_info =
            ash::vk::CommandPoolCreateInfo::builder().queue_family_index(queue_family_index);
        let pool = unsafe {
            base.logical_device.0
                .create_command_pool(&command_pool_info, None)?
        };
        base.name_object(pool, name)?;

        Ok(Self(pool))
    }
}
