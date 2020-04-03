use crate::UrnError;
use crate::Base;

use ash::version::DeviceV1_0;
use ash::version::DeviceV1_2;

pub struct TimeLine(pub ash::vk::Semaphore);

impl TimeLine {
    pub fn new(
        base: &Base,
        initial_value: u64,
        name: String,
    ) -> Result<Self, UrnError> {
        let mut timeline_info = ash::vk::SemaphoreTypeCreateInfo::builder()
            .semaphore_type(ash::vk::SemaphoreType::TIMELINE)
            .initial_value(initial_value);
        let semaphore_info = ash::vk::SemaphoreCreateInfo::builder().push_next(&mut timeline_info);

        let timeline = unsafe { base.logical_device.0.create_semaphore(&semaphore_info, None)? };
        base.name_object(timeline, name)?;

        Ok(Self(timeline))
    }

    pub fn wait(
        &self,
        base: &Base,
        wait_value: u64,
    ) -> Result<(), UrnError> {
        let semaphores = [self.0];
        let values = [wait_value];
        let wait_info = ash::vk::SemaphoreWaitInfo::builder()
            .flags(ash::vk::SemaphoreWaitFlags::ANY)
            .semaphores(&semaphores)
            .values(&values);

        unsafe {
            base.logical_device.0
                .wait_semaphores(
                    base.logical_device.0.handle(),
                    &wait_info,
                    std::u64::MAX,
                )?
        }

        Ok(())
    }

    pub fn signal(
        &self,
        base: &Base,
        signal_value: u64,
    ) -> Result<(), UrnError> {
        let signal_info = ash::vk::SemaphoreSignalInfo::builder()
            .semaphore(self.0)
            .value(signal_value);

        unsafe {
             base.logical_device.0
                .signal_semaphore(
                    base.logical_device.0.handle(),
                    &signal_info,
                )?
        }

        Ok(())
    }

    pub fn query(
        &self,
        base: &Base,
    ) -> Result<u64, UrnError> {
         Ok(unsafe {
             base.logical_device.0
                .get_semaphore_counter_value(
                    base.logical_device.0.handle(),
                    self.0,
                )?
        })
    }
}
