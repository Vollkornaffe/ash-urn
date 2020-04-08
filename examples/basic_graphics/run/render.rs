use crate::AppError;

use ash_urn::Base;
use ash_urn::Command;
use ash_urn::{Timeline, Semaphore, Fence};

use ash::version::DeviceV1_0;

pub fn submit(
    base: &Base,
    graphics_command: &Command,
    timeline: &Timeline,
    semaphore_image_acquired: &Semaphore,
    semaphore_rendering_finished: &Semaphore,
    fence_rendering_finished: &Fence,
    frame: u64,
    image_index: u32,
) -> Result<(), AppError> {

        // choose the buffer corresponding to the image
        let graphics_command_buffers = [graphics_command.buffers[image_index as usize].0];

        // setup waiting / signaling for rendering
        let wait_values = [1];
        let signal_values = [frame + 1, 1];
        let mut timeline_submit_info = ash::vk::TimelineSemaphoreSubmitInfo::builder()
            .wait_semaphore_values(&wait_values)
            .signal_semaphore_values(&signal_values)
            .build();
        let graphics_wait_semaphores = [semaphore_image_acquired.0];
        let graphics_wait_stages_mask = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let graphics_signal_semaphores = [timeline.0, semaphore_rendering_finished.0];

        // setup submit
        let graphics_submit_info = ash::vk::SubmitInfo::builder()
            .wait_semaphores(&graphics_wait_semaphores)
            .wait_dst_stage_mask(&graphics_wait_stages_mask)
            .command_buffers(&graphics_command_buffers)
            .signal_semaphores(&graphics_signal_semaphores)
            .push_next(&mut timeline_submit_info);
        let graphics_submits = [graphics_submit_info.build()];

        unsafe {
            base.logical_device.0.queue_submit(
                graphics_command.queue.0,
                &graphics_submits,
                fence_rendering_finished.0,
            )?;
        }

        Ok(())
}
