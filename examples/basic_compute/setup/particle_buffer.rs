use crate::AppError;

use crate::{Particle, Particles};

use ash_urn::transfer::{create_storage_device_buffer, ownership};
use ash_urn::Base;
use ash_urn::Command;
use ash_urn::DeviceBuffer;

pub fn setup(
    base: &Base,
    particles: &Particles,
    combined_command: &Command,
    transfer_command: &Command,
) -> Result<DeviceBuffer, AppError> {
    let particle_buffer = create_storage_device_buffer::<Particle>(
        &base,
        particles.0.as_slice(),
        transfer_command.queue.0,
        transfer_command.pool.0,
        "ParticleBuffer".to_string(),
    )?;

    // transfer the ownership to the combined queue family
    ownership::transfer_to_combined(
        &base,
        &[&particle_buffer],
        &[],
        &transfer_command,
        &combined_command,
    )?;

    Ok(particle_buffer)
}
