use crate::UrnError;
use crate::Base;

use std::collections::HashMap;

pub mod layout;
pub mod pool;
pub mod set;
pub mod info;
mod descriptor_types;

pub use layout::Layout;
pub use pool::Pool;
pub use set::Set;
use descriptor_types::DESCRIPTOR_TYPES;

pub struct Descriptor {
    pub layout: Layout,
    pub pool: Pool,
    pub sets: Vec<Set>,
}

pub struct Setup {
    pub ty: ash::vk::DescriptorType,
    pub stage: ash::vk::ShaderStageFlags,
}

pub enum Usage {
    Buffer(ash::vk::Buffer),
    ImageSampler(ash::vk::ImageLayout, ash::vk::ImageView, ash::vk::Sampler),
}

pub struct SetUsage {
    pub usages: HashMap<u32, Usage>,
    pub name: String,
}

pub struct DescriptorSettings {
    pub setup_map: HashMap<u32, Setup>,
    pub set_usages: Vec<SetUsage>,
    pub name: String,
}

impl Descriptor {

    pub fn new(
        base: &Base,
        settings: &DescriptorSettings,
    ) -> Result<Self, UrnError> {

        let num_sets = settings.set_usages.len() as u32;

        let bindings: Vec<ash::vk::DescriptorSetLayoutBinding> = settings
            .setup_map
            .iter()
            .map(|(binding, setup)|
                ash::vk::DescriptorSetLayoutBinding::builder()
                    .binding(*binding)
                    .descriptor_type(setup.ty)
                    .descriptor_count(1)
                    .stage_flags(setup.stage)
                    .build()
            )
            .collect();
        let layout = Layout::new(
            base,
            bindings.as_slice(),
            format!("{}Layout", settings.name.clone()),
        )?;

        let pool_sizes: Vec<ash::vk::DescriptorPoolSize> = DESCRIPTOR_TYPES
            .iter()
            .map(|&ty|
                ash::vk::DescriptorPoolSize::builder()
                    .ty(ty)
                    .descriptor_count(
                        settings
                            .setup_map
                            .iter()
                            .map(|(_, s)| if s.ty == ty {
                                num_sets
                            } else {
                                0
                            })
                            .sum()
                    )
                    .build()
            )
            .collect();
        let pool = Pool::new(
            base,
            pool_sizes.as_slice(),
            num_sets,
            format!("{}Pool", settings.name.clone()),
        )?;

        let mut sets = Vec::new();
        for set_usage in &settings.set_usages {
            sets.push(Set::new(
                base,
                layout.0,
                pool.0,
                &settings.setup_map,
                &set_usage,
            )?);
        }
                
        Ok(Self {
            layout,
            pool,
            sets,
        })

    }

}
