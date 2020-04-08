use crate::AppError;

use ash_urn::base::queue_families::{COMBINED, DEDICATED_TRANSFER};
use ash_urn::Base;
use ash_urn::{Command, CommandSettings};

pub fn setup(base: &Base, n_buffer: u32) -> Result<(Command, Command), AppError> {
    let transfer_queue_family_idx = base.queue_map.get(&DEDICATED_TRANSFER).unwrap().idx;
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

    // Transfer no buffers allocated, because one-time commands only
    let transfer_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: transfer_queue_family_idx,
            n_buffer: 0,
            name: "TransferCommand".to_string(),
        },
    )?;

    Ok((graphics_command, transfer_command))
}
