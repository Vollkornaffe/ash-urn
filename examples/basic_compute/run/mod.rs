use crate::AppError;
use crate::Setup;

pub mod compute;
pub mod next_image;
pub mod present;
pub mod render;
pub mod uniform_buffer;

use ash_urn::Base;

pub fn advance_frame(
    base: &Base,
    setup: &Setup,
    start_instant: &std::time::Instant,
    time: &mut u64,
    profiling: bool,
) -> Result<(), AppError> {
    // wait for last frame to complete rendering before submitting.
    setup.timeline.wait(&base, *time)?;

    // only waiting on fence because the validation layers don't get timelines
    // if running without validation, the fence is not needed.
    setup.fence_rendering_finished.wait(&base)?;
    setup.fence_rendering_finished.reset(&base)?;

    //if *time != 0 {
    if false {
        let stamps = setup.timestamp.query_all(base)?;
        println!("CALCULATE: {}", 1.0e-6 * (stamps[1] - stamps[0]) as f64);
        println!("INTEGRATE: {}", 1.0e-6 * (stamps[3] - stamps[2]) as f64);
        println!("RENDER:    {}", 1.0e-6 * (stamps[5] - stamps[4]) as f64);
    }

    // run computation
    compute::submit(&base, &setup.compute_command, &setup.timeline, *time)?;
    *time += 1;

    // acquire an image
    let image_index = next_image::aquire(&setup.swap_chain, &setup.semaphore_image_acquired)?;

    // update model matrix based on time
    uniform_buffer::update_graphics(
        &base,
        &setup.graphics_uniform_buffers[image_index as usize],
        &setup.swap_chain,
        &start_instant,
    )?;

    // submit the rendering commands to the combined queue
    // waiting on image_aquired, signaling rendering_finished
    render::submit(
        &base,
        &setup.graphics_command,
        &setup.timeline,
        &setup.semaphore_image_acquired,
        &setup.semaphore_rendering_finished,
        &setup.fence_rendering_finished,
        *time,
        image_index,
    )?;
    *time += 1;

    // submit to the present queue via the swap chain loader
    // waiting on rendering_finished, doesn't signal anything
    present::submit(
        &setup.swap_chain,
        &setup.graphics_command,
        &setup.semaphore_rendering_finished,
        image_index,
    )?;

    Ok(())
}
