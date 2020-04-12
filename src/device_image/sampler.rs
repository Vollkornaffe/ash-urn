use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Sampler(pub ash::vk::Sampler);

impl Sampler {
    pub fn new(base: &Base, name: String) -> Result<Self, UrnError> {
        let sampler_info = ash::vk::SamplerCreateInfo::builder()
            .mag_filter(ash::vk::Filter::LINEAR)
            .min_filter(ash::vk::Filter::LINEAR)
            .address_mode_u(ash::vk::SamplerAddressMode::REPEAT)
            .address_mode_v(ash::vk::SamplerAddressMode::REPEAT)
            .address_mode_w(ash::vk::SamplerAddressMode::REPEAT)
            .anisotropy_enable(false)
            .max_anisotropy(0 as f32)
            .border_color(ash::vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(ash::vk::CompareOp::ALWAYS)
            .mipmap_mode(ash::vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(0.0);

        let sampler = unsafe { base.logical_device.0.create_sampler(&sampler_info, None)? };
        base.name_object(sampler, name)?;
        Ok(Self(sampler))
    }
    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_sampler(self.0, None);
        }
    }
}
