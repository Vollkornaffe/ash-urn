pub mod assets;
pub mod error;
pub mod particles;
pub mod run;
pub mod sdl;
pub mod setup;

pub use error::AppError;
pub use particles::{Particle, Particles};
pub use sdl::SDL;
pub use setup::Setup;

use ash_urn::memory_alignment::Align16;
use ash_urn::wait_device_idle;

#[repr(C)]
#[derive(Debug)]
struct GraphicsUBO {
    model: Align16<cgmath::Matrix4<f32>>,
    view: Align16<cgmath::Matrix4<f32>>,
    proj: Align16<cgmath::Matrix4<f32>>,
}

#[repr(C)]
#[derive(Debug)]
struct ComputeUBO {
    n_particles: u32,
    n_reference: u32,
    scale: f32,
    d_t: f32,
    G: f32,
}

fn main() {
    println!("Starting basic_compute.");

    // create particles
    let particles = Particles::new(25);

    // load reference mesh
    let reference_mesh = &assets::load_mesh("examples/basic_graphics/assets/test.glb").unwrap();

    // create sdl context
    let mut sdl = sdl::SDL::new(sdl::WindowSettings {
        title: "Basic Compute",
        w: 800,
        h: 800,
        maximized: true,
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
            &reference_mesh,
            &particles,
        )
        .unwrap();

        // record starting time
        let start_instant = std::time::Instant::now();
        let mut frame = 0;
        'running: loop {
            for e in sdl.get_events() {
                match e {
                    sdl::SdlEvent::Close => break 'running,
                }
            }

            // check if the iteration failed due to resize
            match run::advance_frame(&base, &setup, &start_instant, &mut frame, false) {
                Err(AppError::AshError(ash::vk::Result::ERROR_OUT_OF_DATE_KHR)) => {
                    setup = Setup::new(
                        &sdl,
                        &base,
                        &surface_loader,
                        surface,
                        &reference_mesh,
                        &particles,
                    )
                    .unwrap();
                    frame = 0;
                    Ok(())
                }
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
