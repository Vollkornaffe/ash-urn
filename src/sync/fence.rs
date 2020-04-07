use crate::Base;
use crate::UrnError;

use ash::version::DeviceV1_0;

pub struct Fence(pub ash::vk::Fence);

impl Fence {
    pub fn new(base: &Base, signaled: bool, name: String) -> Result<Self, UrnError> {
        let fence_info = if signaled {
            ash::vk::FenceCreateInfo::builder().flags(ash::vk::FenceCreateFlags::SIGNALED)
        } else {
            ash::vk::FenceCreateInfo::builder()
        };
        let fence = unsafe { base.logical_device.0.create_fence(&fence_info, None)? };
        base.name_object(fence, name)?;
        Ok(Self(fence))
    }

    pub fn wait(&self, base: &Base) -> Result<(), UrnError> {
        let fences = [self.0];
        unsafe {
            base.logical_device
                .0
                .wait_for_fences(&fences, true, std::u64::MAX)?
        };
        Ok(())
    }

    pub fn reset(&self, base: &Base) -> Result<(), UrnError> {
        let fences = [self.0];
        unsafe { base.logical_device.0.reset_fences(&fences)? };
        Ok(())
    }

    pub fn query(&self, base: &Base) -> Result<bool, UrnError> {
        Ok(unsafe { base.logical_device.0.get_fence_status(self.0)? })
    }

    pub fn destroy(&self, base: &Base) {
        unsafe {
            base.logical_device.0.destroy_fence(self.0, None);
        }
    }
}
