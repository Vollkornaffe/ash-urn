use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;

pub struct Timestamp {
    pub pool: ash::vk::QueryPool,
    pub names: Vec<String>,
}

impl Timestamp {
    pub fn new(
        base: &Base,
        stamp_names: Vec<String>,
        name: String,
    ) -> Result<Self, UrnError> {

        let query_pool_info = ash::vk::QueryPoolCreateInfo::builder()
            .query_type(ash::vk::QueryType::TIMESTAMP)
            .query_count(stamp_names.len() as u32);
        let query_pool = unsafe {
            base.logical_device.0.create_query_pool(
                &query_pool_info,
                None,
            )?
        };
        base.name_object(query_pool, name)?;

        Ok(Self {
            pool: query_pool,
            names: stamp_names,
        })
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_query_pool(self.pool, None);
        }       
    }
}