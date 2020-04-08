pub mod error;
pub mod sdl;

pub use error::AppError;
pub use sdl::SDL;
pub use setup::Setup;

mod run;
mod setup;

use ash_urn::memory_alignment::Align16;
use ash_urn::wait_device_idle;
use ash_urn::Base;
use ash_urn::Mesh;

#[repr(C)]
struct UBO {
    model: Align16<cgmath::Matrix4<f32>>,
    view: Align16<cgmath::Matrix4<f32>>,
    proj: Align16<cgmath::Matrix4<f32>>,
}

fn advance_frame(
    base: &Base,
    setup: &Setup,
    start_instant: &std::time::Instant,
    frame: &mut u64,
) -> Result<(), AppError> {
    // wait for last frame to complete rendering before submitting.
    setup.timeline.wait(&base, *frame)?;

    // acquire an image
    let image_index = run::next_image::aquire(&setup.swap_chain, &setup.semaphore_image_acquired)?;

    // only waiting on fence because the validation layers don't get timelines
    // if running without validation, the fence is not needed.
    setup.fence_rendering_finished.wait(&base)?;
    setup.fence_rendering_finished.reset(&base)?;

    // update model matrix based on time
    run::uniform_buffer::update(
        &base,
        &setup.uniform_buffers[image_index as usize],
        &setup.swap_chain,
        &start_instant,
    )?;

    // submit the rendering commands to the combined queue
    // waiting on image_aquired, signaling rendering_finished
    run::render::submit(
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
    run::present::submit(
        &setup.swap_chain,
        &setup.graphics_command,
        &setup.semaphore_rendering_finished,
        image_index,
    )?;

    *frame += 1;

    Ok(())
}

fn main() {
    println!("Starting basic_graphics.");

    // create a mesh to render
    let mesh = Mesh::new().add_quad(
        [-1.0, -1.0, 0.0],
        [1.0, -1.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 1.0],
    );

    // create sdl context
    let mut sdl = sdl::SDL::new(sdl::WindowSettings {
        title: "Basic Graphics",
        w: 800,
        h: 800,
        maximized: false,
    })
    .unwrap();

    // setup the basic vulkan stuff, this is convoluted with
    // surface stuff, can't really be separated further
    let (base, surface_loader, surface) = setup::base::setup(&mut sdl).unwrap();

    // this scope is to ensure base, surface_loader & surface outlive whats inside
    {

        let mut setup = Setup::new(
            &sdl,
            &base,
            &surface_loader,
            surface,
            &mesh,
        ).unwrap();

        // and we wait until device is idle before we start the actual main loop
        wait_device_idle(&base).unwrap();

        // record starting time
        let start_instant = std::time::Instant::now();
        let mut frame = 0;
        'running: loop {
            for e in sdl.get_events() {
                match e {
                    sdl::SdlEvent::Close => break 'running,
                    sdl::SdlEvent::Resize => {
                        wait_device_idle(&base).unwrap();
                        setup = Setup::new(
                            &sdl,
                            &base,
                            &surface_loader,
                            surface,
                            &mesh,
                        ).unwrap();
                        wait_device_idle(&base).unwrap();
                        frame = 0;
                    },
                }
            }

            // check if the iteration failed due to resize
            match advance_frame(
                &base,
                &setup,
                &start_instant,
                &mut frame,
            ) {
                Err(AppError::AshError(ash::vk::Result::ERROR_OUT_OF_DATE_KHR)) => Ok(()),
                x => x,
            }
            .unwrap();
        }

        // wait until everything is done before we start deconstruction
        wait_device_idle(&base).unwrap();

    }

    unsafe {
        surface_loader.destroy_surface(surface, None);
    }
}
