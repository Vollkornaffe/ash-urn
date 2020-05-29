use crate::AppError;

use ash_urn::base::queue_families::{COMBINED, DEDICATED_TRANSFER};
use ash_urn::Base;
use ash_urn::{Command, CommandBuffer, CommandSettings};

pub fn setup(base: &Base, n_buffer: u32) -> Result<(Command, Vec<CommandBuffer>, Command), AppError> {
    let graphics_command = setup_graphics(base)?;
    let graphics_command_buffers = CommandBuffer::alloc_vec(
        base,
        graphics_command.pool.0,
        n_buffer,
        "GraphicsCommandBuffer".to_string(),
    )?;
    let transfer_command = setup_transfer(base)?;

    Ok((graphics_command, graphics_command_buffers, transfer_command))
}

pub fn setup_graphics(base: &Base) -> Result<Command, AppError> {
    let combined_queue_family_idx = base.queue_map.get(&COMBINED).unwrap().idx;

    // Create graphic commands, one buffer per image
    let graphics_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: combined_queue_family_idx,
            queue_idx: 0,
            name: "GraphicsCommand".to_string(),
        },
    )?;

    Ok(graphics_command)
}

pub fn setup_transfer(base: &Base) -> Result<Command, AppError> {
    let transfer_queue_family_idx = base.queue_map.get(&DEDICATED_TRANSFER).unwrap().idx;

    // Transfer no buffers allocated, because one-time commands only
    let transfer_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: transfer_queue_family_idx,
            queue_idx: 0,
            name: "TransferCommand".to_string(),
        },
    )?;

    Ok(transfer_command)
}
