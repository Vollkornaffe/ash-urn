use crate::AppError;
use crate::UBO;

use ash_urn::Base;
use ash_urn::DeviceBuffer;
use ash_urn::SwapChain;

pub fn update(
    base: &Base,
    uniform_buffer: &DeviceBuffer,
    swap_chain: &SwapChain,
    start_instant: &std::time::Instant,
) -> Result<(), AppError> {
    // prepare uniform buffer wrt. time
    let t = start_instant.elapsed().as_secs_f32();

    let rotation: cgmath::Quaternion<f32> = cgmath::Rotation3::from_angle_z(cgmath::Rad(t));

    let model: cgmath::Matrix4<f32> = rotation.into();

    let view = cgmath::Matrix4::look_at(
        cgmath::Point3::new(4.0, 4.0, 4.0),
        cgmath::Point3::new(0.0, 0.0, 0.0),
        cgmath::Vector3::unit_z(),
    );

    let mut proj = cgmath::perspective(
        cgmath::Deg(45.0),
        swap_chain.extent.0.width as f32 / swap_chain.extent.0.height as f32,
        0.1,
        100.0,
    );
    proj[1][1] *= -1.0;

    uniform_buffer.write(
        &base,
        &UBO {
            model: model.into(),
            view: view.into(),
            proj: proj.into(),
        },
    )?;

    Ok(())
}
