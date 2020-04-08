pub mod error;
pub mod sdl;
pub mod run;
pub mod setup;

pub use error::AppError;
pub use sdl::SDL;
pub use setup::Setup;

use ash_urn::memory_alignment::Align16;
use ash_urn::wait_device_idle;
use ash_urn::Mesh;

#[repr(C)]
struct UBO {
    model: Align16<cgmath::Matrix4<f32>>,
    view: Align16<cgmath::Matrix4<f32>>,
    proj: Align16<cgmath::Matrix4<f32>>,
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
            match run::advance_frame(
                &base,
                &setup,
                &start_instant,
                &mut frame,
            ) {
                Err(AppError::AshError(ash::vk::Result::ERROR_OUT_OF_DATE_KHR)) => {
                    setup = Setup::new(
                        &sdl,
                        &base,
                        &surface_loader,
                        surface,
                        &mesh,
                    ).unwrap();
                    frame = 0;
                    Ok(())
                },
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
