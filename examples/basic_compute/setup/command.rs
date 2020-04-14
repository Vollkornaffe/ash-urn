use crate::AppError;

use ash_urn::base::queue_families::{COMBINED, DEDICATED_TRANSFER};
use ash_urn::Base;
use ash_urn::{Command, CommandSettings};
use ash_urn::{PipelineLayout, ComputePipeline};
use ash_urn::Descriptor;

use ash::version::DeviceV1_0;

pub fn setup(base: &Base, n_buffer: u32) -> Result<(Command, Command, Command), AppError> {
    let graphics_command = setup_graphics(base, n_buffer)?;
    let compute_command = setup_compute(base)?;
    let transfer_command = setup_transfer(base)?;

    Ok((graphics_command, compute_command, transfer_command))
}

pub fn setup_graphics(base: &Base, n_buffer: u32) -> Result<Command, AppError> {
    let combined_queue_family_idx = base.queue_map.get(&COMBINED).unwrap().idx;

    // Create graphic commands, one buffer per image
    let graphics_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: combined_queue_family_idx,
            n_buffer: n_buffer,
            name: "GraphicsCommand".to_string(),
        },
    )?;

    Ok(graphics_command)
}

pub fn setup_compute(base: &Base) -> Result<Command, AppError> {
    let combined_queue_family_idx = base.queue_map.get(&COMBINED).unwrap().idx;

    // Create compute commands, just one buffer
    let compute_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: combined_queue_family_idx,
            n_buffer: 1,
            name: "ComputeCommand".to_string(),
        },
    )?;

    Ok(compute_command)
}

pub fn setup_transfer(base: &Base) -> Result<Command, AppError> {
    let transfer_queue_family_idx = base.queue_map.get(&DEDICATED_TRANSFER).unwrap().idx;

    // Transfer no buffers allocated, because one-time commands only
    let transfer_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: transfer_queue_family_idx,
            n_buffer: 0,
            name: "TransferCommand".to_string(),
        },
    )?;

    Ok(transfer_command)
}

pub fn write_compute(
    base: &Base,
    pipeline_layout: &PipelineLayout,
    calculate_pipeline: &ComputePipeline,
    integrate_pipeline: &ComputePipeline,
    command: &Command,
    descriptor: &Descriptor,
    n_particles: u32,
) -> Result<(), AppError> {

    let command_buffer = command.buffers[0].0;

    let begin_info = ash::vk::CommandBufferBeginInfo::builder();

    let descriptor_sets = [descriptor.sets[0].0];
    let dynamic_offsets = [];

    let memory_barriers = [ash::vk::MemoryBarrier::builder()
        .src_access_mask(ash::vk::AccessFlags::SHADER_READ | ash::vk::AccessFlags::SHADER_WRITE)
        .dst_access_mask(ash::vk::AccessFlags::SHADER_READ | ash::vk::AccessFlags::SHADER_WRITE)
        .build()];
    let buffer_memory_barriers = [];
    let image_memory_barriers = [];

    unsafe {

        base.logical_device.0
            .begin_command_buffer(command_buffer, &begin_info)?;

        base.logical_device.0.cmd_bind_descriptor_sets(
            command_buffer,
            ash::vk::PipelineBindPoint::COMPUTE,
            pipeline_layout.0,
            0,
            &descriptor_sets,
            &dynamic_offsets,
        );

        base.logical_device.0.cmd_bind_pipeline(
            command_buffer,
            ash::vk::PipelineBindPoint::COMPUTE,
            calculate_pipeline.0,
        );
        base.logical_device.0.cmd_dispatch(
            command_buffer,
            1 + n_particles / 512,
            1,
            1,
        );

        base.logical_device.0.cmd_pipeline_barrier(
            command_buffer,
            ash::vk::PipelineStageFlags::COMPUTE_SHADER,
            ash::vk::PipelineStageFlags::COMPUTE_SHADER,
            ash::vk::DependencyFlags::default(),
            &memory_barriers,
            &buffer_memory_barriers,
            &image_memory_barriers,
        );

        base.logical_device.0.cmd_bind_pipeline(
            command_buffer,
            ash::vk::PipelineBindPoint::COMPUTE,
            integrate_pipeline.0,
        );
        base.logical_device.0.cmd_dispatch(
            command_buffer,
            1 + n_particles / 512,
            1,
            1,
        );

        base.logical_device.0.end_command_buffer(command_buffer)?;
    }
    Ok(())
}
