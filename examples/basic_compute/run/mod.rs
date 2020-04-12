use crate::AppError;
use crate::Setup;

pub mod next_image;
pub mod present;
pub mod render;
pub mod uniform_buffer;

use ash_urn::Base;

pub fn advance_frame(
    base: &Base,
    setup: &Setup,
    start_instant: &std::time::Instant,
    frame: &mut u64,
) -> Result<(), AppError> {
    // acquire an image
    let image_index = next_image::aquire(&setup.swap_chain, &setup.semaphore_image_acquired)?;

    // wait for last frame to complete rendering before submitting.
    setup.timeline.wait(&base, *frame)?;

    // only waiting on fence because the validation layers don't get timelines
    // if running without validation, the fence is not needed.
    setup.fence_rendering_finished.wait(&base)?;
    setup.fence_rendering_finished.reset(&base)?;

    /*
    if *frame != 0 {
        let stamps = setup.timestamp.query_all(base)?;
        println!("{:?}", stamps[1] - stamps[0]);
    }
    */

    // update model matrix based on time
    uniform_buffer::update(
        &base,
        &setup.uniform_buffers[image_index as usize],
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
        *frame,
        image_index,
    )?;

    // submit to the present queue via the swap chain loader
    // waiting on rendering_finished, doesn't signal anything
    present::submit(
        &setup.swap_chain,
        &setup.graphics_command,
        &setup.semaphore_rendering_finished,
        image_index,
    )?;

    *frame += 1;

    Ok(())
}
