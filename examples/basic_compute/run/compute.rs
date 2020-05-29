use crate::AppError;

use ash_urn::Base;
use ash_urn::Command;
use ash_urn::CommandBuffer;
use ash_urn::Timeline;

use ash::version::DeviceV1_0;

pub fn submit(
    base: &Base,
    compute_command: &Command,
    compute_command_buffer: &CommandBuffer,
    timeline: &Timeline,
    time: u64,
) -> Result<(), AppError> {
    let compute_command_buffers = [compute_command_buffer.0];

    // setup waiting / signaling for computing
    let wait_values = [time];
    let signal_values = [time + 1];
    let mut timeline_submit_info = ash::vk::TimelineSemaphoreSubmitInfo::builder()
        .wait_semaphore_values(&wait_values)
        .signal_semaphore_values(&signal_values)
        .build();
    let compute_wait_semaphores = [timeline.0];
    let compute_wait_stages_mask = [ash::vk::PipelineStageFlags::COMPUTE_SHADER];
    let compute_signal_semaphores = [timeline.0];

    // setup submit
    let compute_submit_info = ash::vk::SubmitInfo::builder()
        .wait_semaphores(&compute_wait_semaphores)
        .wait_dst_stage_mask(&compute_wait_stages_mask)
        .command_buffers(&compute_command_buffers)
        .signal_semaphores(&compute_signal_semaphores)
        .push_next(&mut timeline_submit_info);

    let compute_submits = [compute_submit_info.build()];

    unsafe {
        base.logical_device.0.queue_submit(
            compute_command.queue.0,
            &compute_submits,
            ash::vk::Fence::default(),
        )?;
    }

    Ok(())
}
