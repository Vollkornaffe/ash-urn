use crate::AppError;

use ash_urn::transfer::{
    create_index_device_buffer, create_vertex_storage_device_buffer, ownership,
};
use ash_urn::Base;
use ash_urn::Command;
use ash_urn::DeviceBuffer;
use ash_urn::Mesh;

pub fn setup(
    base: &Base,
    mesh: &Mesh,
    graphics_command: &Command,
    transfer_command: &Command,
) -> Result<(DeviceBuffer, DeviceBuffer), AppError> {
    // create vertex buffer
    let vertex_device_buffer = create_vertex_storage_device_buffer(
        &base,
        mesh.vertices.as_slice(),
        transfer_command.queue.0,
        transfer_command.pool.0,
        "VertexBuffer".to_string(),
    )?;

    // create index buffer
    let index_device_buffer = create_index_device_buffer(
        &base,
        mesh.indices.as_slice(),
        transfer_command.queue.0,
        transfer_command.pool.0,
        "IndexBuffer".to_string(),
    )?;

    // transfer the ownership to the combined queue family
    ownership::transfer_to_combined(
        &base,
        &[&vertex_device_buffer, &index_device_buffer],
        &[],
        &transfer_command,
        &graphics_command, // any command struct from the combined family is ok
    )?;

    Ok((vertex_device_buffer, index_device_buffer))
}
