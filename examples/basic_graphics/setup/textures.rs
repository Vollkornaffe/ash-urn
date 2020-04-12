use crate::AppError;

use ash_urn::transfer::{create_texture_device_image, ownership};
use ash_urn::Base;
use ash_urn::Command;
use ash_urn::DeviceImage;

pub fn setup(
    base: &Base,
    files_and_names: &[(String, String)],
    graphics_command: &Command,
    transfer_command: &Command,
) -> Result<Vec<DeviceImage>, AppError> {

    let mut texture_device_images = Vec::new();

    for (filename, name) in files_and_names {
        texture_device_images.push(create_texture_device_image(
            base,
            filename.clone(),
            transfer_command.queue.0,
            transfer_command.pool.0,
            name.clone(),
        )?);
    }

    let refs: Vec<&DeviceImage> = texture_device_images.iter().map(|d| d).collect();
    ownership::transfer_to_combined(
        &base,
        &[],
        &refs,
        &transfer_command,
        &graphics_command,
    )?;

    Ok(texture_device_images)
}
