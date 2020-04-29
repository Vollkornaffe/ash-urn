use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Timestamp {
    pub pool: ash::vk::QueryPool,
    pub names: Vec<String>,
    pub timestamp_period: f32,
}

impl Timestamp {
    pub fn new(
        base: &Base,
        stamp_names: Vec<String>,
        timestamp_period: f32,
        name: String,
    ) -> Result<Self, UrnError> {
        let query_pool_info = ash::vk::QueryPoolCreateInfo::builder()
            .query_type(ash::vk::QueryType::TIMESTAMP)
            .query_count(stamp_names.len() as u32);
        let query_pool = unsafe {
            base.logical_device
                .0
                .create_query_pool(&query_pool_info, None)?
        };
        base.name_object(query_pool, name)?;

        Ok(Self {
            pool: query_pool,
            names: stamp_names,
            timestamp_period,
        })
    }

    pub fn reset_pool(&self, base: &Base, command_buffer: ash::vk::CommandBuffer) {
        unsafe {
            base.logical_device.0.cmd_reset_query_pool(
                command_buffer,
                self.pool,
                0,
                self.names.len() as u32,
            )
        }
    }

    pub fn mark(
        &self,
        base: &Base,
        command_buffer: ash::vk::CommandBuffer,
        pipeline_stage: ash::vk::PipelineStageFlags,
        name: &str,
    ) {
        let query_idx = self
            .names
            .iter()
            .position(|n| n == &name.to_string())
            .expect("Timestamp name not found.") as u32;
        unsafe {
            base.logical_device.0.cmd_write_timestamp(
                command_buffer,
                pipeline_stage,
                self.pool,
                query_idx,
            );
        }
    }

    pub fn query_all(&self, base: &Base) -> Result<Vec<u64>, UrnError> {
        let mut data: Vec<u64> = Vec::new();
        data.resize(self.names.len(), 0);
        unsafe {
            base.logical_device.0.get_query_pool_results(
                self.pool,
                0,
                self.names.len() as u32,
                &mut data,
                ash::vk::QueryResultFlags::TYPE_64,
            )?;
        }
        for d in &mut data {
            *d *= self.timestamp_period as u64;
        }
        Ok(data)
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_query_pool(self.pool, None);
        }
    }
}
