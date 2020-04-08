pub mod sdl;
pub mod error;

pub use sdl::SDL;
pub use error::AppError;

mod setup;
mod run;

use ash_urn::Base;
use ash_urn::DeviceBuffer;
use ash_urn::SwapChain;
use ash_urn::Command;
use ash_urn::{Timeline, Semaphore, Fence};
use ash_urn::memory_alignment::Align16;
use ash_urn::Mesh;
use ash_urn::command;
use ash_urn::wait_device_idle;

use ash::version::DeviceV1_0;

#[repr(C)]
struct UBO {
    model: Align16<cgmath::Matrix4<f32>>,
    view: Align16<cgmath::Matrix4<f32>>,
    proj: Align16<cgmath::Matrix4<f32>>,
}

fn advance_frame(
    base: &Base,
    swap_chain: &SwapChain,
    graphics_command: &Command,
    uniform_buffers: &[DeviceBuffer],
    timeline: &Timeline,
    semaphore_image_acquired: &Semaphore,
    semaphore_rendering_finished: &Semaphore,
    fence_rendering_finished: &Fence,
    start_instant: &std::time::Instant,
    frame: &mut u64,
    image_index: &mut u32,
) -> Result<(), AppError> {

    // wait for last frame to complete rendering before submitting.
    timeline.wait(&base, *frame)?;

    // only waiting on fence because the validation layers don't get timelines
    // if running without validation, the fence is not needed.
    fence_rendering_finished.wait(&base)?;
    fence_rendering_finished.reset(&base)?;

    // update model matrix based on time
    run::uniform_buffer::update(
        &base,
        &uniform_buffers[*image_index as usize],
        &swap_chain,
        &start_instant,
    )?;

    // submit the rendering commands to the combined queue
    // waiting on image_aquired, signaling rendering_finished
    run::render::submit(
        &base,
        &graphics_command,
        &timeline,
        &semaphore_image_acquired,
        &semaphore_rendering_finished,
        &fence_rendering_finished,
        *frame,
        *image_index,
    )?;

    // submit to the present queue via the swap chain loader
    // waiting on rendering_finished, doesn't signal anything
    run::present::submit(
        &swap_chain,
        &graphics_command,
        &semaphore_rendering_finished,
        *image_index,
    )?;

    // acquire an image for the next iteration
    *image_index = run::next_image::aquire(
        &swap_chain,
        &semaphore_image_acquired,
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

    // get swap chain + renderpass & depth image
    // this is also a bit entangled
    let (swap_chain, render_pass, depth_device_image) = setup::swap_chain::setup(
        &base,
        &sdl,
        &surface_loader,
        surface,
    ).unwrap();

    // an uniform buffer per swapchain image
    let uniform_buffers = setup::uniform_buffers::setup(&base, swap_chain.image_count).unwrap();

    // these sets only contain the respective UBO
    let descriptor = setup::descriptor::setup(&base, &uniform_buffers).unwrap();

    // get the structures for commands,
    // they will be filled out later
    let (graphics_command, transfer_command) = setup::command::setup(&base, swap_chain.image_count).unwrap();

    // create device buffers from the mesh
    // the transfer is done with the transfer command,
    // ownership is transferred afterwards
    let (vertex_device_buffer, index_device_buffer) = setup::mesh_buffers::setup(
        &base,
        &mesh,
        &graphics_command,
        &transfer_command,
    ).unwrap();

    // just one pipeline, using the vert & frag shader
    let (graphics_pipeline_layout, graphics_pipeline) = setup::pipeline::setup(
        &base,
        &descriptor,
        &swap_chain,
        &render_pass,
    ).unwrap();

    // write to the command buffers
    for (i, command_buffer) in graphics_command.buffers.iter().enumerate() {
        command::draw::indexed(
            &base,
            &command::DrawIndexedSettings {
                command_buffer: command_buffer.0,
                render_pass: render_pass.0,
                frame_buffer: swap_chain.elements[i].frame_buffer,
                extent: swap_chain.extent.0,
                graphics_pipeline: graphics_pipeline.0,
                graphics_pipeline_layout: graphics_pipeline_layout.0,
                descriptor_set: descriptor.sets[i].0,
                vertex_buffer: vertex_device_buffer.buffer.0,
                index_buffer: index_device_buffer.buffer.0,
                n_indices: mesh.indices.len() as u32 * 3,
            },
        )
        .unwrap();
    }

    // create all synchronization structs
    let (timeline, semaphore_image_acquired, semaphore_rendering_finished, fence_rendering_finished) = setup::sync::setup(&base).unwrap();

    // the first image index is retrieved
    let mut image_index = run::next_image::aquire(
        &swap_chain,
        &semaphore_image_acquired,
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
                sdl::SdlEvent::Resize => break 'running, //TODO
            }
        }

        // check if the iteration failed due to resize
        match advance_frame(
            &base,
            &swap_chain,
            &graphics_command,
            &uniform_buffers,
            &timeline,
            &semaphore_image_acquired,
            &semaphore_rendering_finished,
            &fence_rendering_finished,
            &start_instant,
            &mut frame,
            &mut image_index,
        ) {
            Err(AppError::AshError(ash::vk::Result::ERROR_OUT_OF_DATE_KHR)) => {
                println!("RESIZE NEEDED");
                Ok(())
            },
            x => x,
        }.unwrap();
    }

    wait_device_idle(&base).unwrap();

    graphics_command.destroy(&base);
    transfer_command.destroy(&base);
    semaphore_image_acquired.destroy(&base);
    semaphore_rendering_finished.destroy(&base);
    timeline.destroy(&base);
    fence_rendering_finished.destroy(&base);
    vertex_device_buffer.destroy(&base);
    index_device_buffer.destroy(&base);
    depth_device_image.destroy(&base);
    for uniform_buffer in uniform_buffers {
        uniform_buffer.destroy(&base);
    }
    swap_chain.destroy(&base);
    descriptor.destroy(&base);

    unsafe {
        base.logical_device
            .0
            .destroy_pipeline_layout(graphics_pipeline_layout.0, None);
        base.logical_device
            .0
            .destroy_pipeline(graphics_pipeline.0, None);
        base.logical_device
            .0
            .destroy_render_pass(render_pass.0, None);
        swap_chain
            .loader
            .0
            .destroy_swapchain(swap_chain.handle, None);
        surface_loader.destroy_surface(surface, None);
    }
}
