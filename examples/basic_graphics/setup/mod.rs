pub mod base;

mod command;
mod descriptor;
mod mesh_buffers;
mod pipeline;
mod swap_chain;
mod sync;
mod textures;
mod uniform_buffers;

use crate::AppError;
use crate::SDL;

use ash_urn::sync::wait_device_idle;
use ash_urn::Base;
use ash_urn::Command;
use ash_urn::Descriptor;
use ash_urn::DeviceBuffer;
use ash_urn::DeviceImage;
use ash_urn::Sampler;
use ash_urn::Fence;
use ash_urn::GraphicsPipeline;
use ash_urn::Mesh;
use ash_urn::PipelineLayout;
use ash_urn::RenderPass;
use ash_urn::Semaphore;
use ash_urn::SwapChain;
use ash_urn::Timeline;
use ash_urn::Timestamp;

pub struct Setup<'a> {
    pub base: &'a Base,
    pub swap_chain: SwapChain,
    pub render_pass: RenderPass,
    pub depth_device_image: DeviceImage,
    pub uniform_buffers: Vec<DeviceBuffer>,
    pub descriptor: Descriptor,
    pub graphics_command: Command,
    pub transfer_command: Command,
    pub vertex_device_buffer: DeviceBuffer,
    pub index_device_buffer: DeviceBuffer,
    pub graphics_pipeline_layout: PipelineLayout,
    pub graphics_pipeline: GraphicsPipeline,
    pub timeline: Timeline,
    pub semaphore_image_acquired: Semaphore,
    pub semaphore_rendering_finished: Semaphore,
    pub fence_rendering_finished: Fence,
    pub timestamp: Timestamp,
    pub textures: Vec<(DeviceImage, Sampler)>,
}

impl<'a> Setup<'a> {
    pub fn new(
        sdl: &SDL,
        base: &'a Base,
        surface_loader: &ash::extensions::khr::Surface,
        surface: ash::vk::SurfaceKHR,
        mesh: &Mesh,
    ) -> Result<Self, AppError> {
        wait_device_idle(base)?;

        // get swap chain + renderpass & depth image
        // this is also a bit entangled
        let (swap_chain, render_pass, depth_device_image) =
            swap_chain::setup(base, &sdl, &surface_loader, surface)?;

        // an uniform buffer per swapchain image
        let uniform_buffers = uniform_buffers::setup(base, swap_chain.image_count)?;

        // get the structures for commands,
        // they will be filled out later
        let (graphics_command, transfer_command) = command::setup(base, swap_chain.image_count)?;

        // create device buffers from the mesh & load the textures
        // the transfer is done with the transfer command,
        // ownership is transferred afterwards
        let (vertex_device_buffer, index_device_buffer) =
            mesh_buffers::setup(base, &mesh, &graphics_command, &transfer_command)?;
        let textures = textures::setup(
            base,
            &[(
                "examples/basic_graphics/assets/meme.jpg".to_string(),
                "MuskyBoy".to_string(),
            )],
            &graphics_command,
            &transfer_command,
        )?;

        // these sets contain the respective UBOs & combined image samplers
        let descriptor = descriptor::setup(base, &uniform_buffers, &textures[0])?;

        // just one pipeline, using the vert & frag shader
        let (graphics_pipeline_layout, graphics_pipeline) =
            pipeline::setup(base, &descriptor, &swap_chain, &render_pass)?;

        // get timestamp for profiling
        let timestamp = Timestamp::new(
            &base,
            vec!["Start".to_string(), "Done".to_string()],
            base.physical_device.timestamp_period(&base.instance.0)?,
            "Timestamp".to_string(),
        )?;

        // write to the command buffers
        for (i, command_buffer) in graphics_command.buffers.iter().enumerate() {
            ash_urn::command::draw::indexed(
                base,
                &ash_urn::command::DrawIndexedSettings {
                    command_buffer: command_buffer.0,
                    timestamp: &timestamp,
                    render_pass: render_pass.0,
                    frame_buffer: swap_chain.elements[i].frame_buffer,
                    extent: swap_chain.extent.0,
                    graphics_pipeline: graphics_pipeline.0,
                    graphics_pipeline_layout: graphics_pipeline_layout.0,
                    descriptor_set: descriptor.sets[i].0,
                    vertex_buffer: vertex_device_buffer.buffer.0,
                    index_buffer: index_device_buffer.buffer.0,
                    n_indices: mesh.indices.len() as u32,
                },
            )?;
        }

        // create all synchronization structs
        let (
            timeline,
            semaphore_image_acquired,
            semaphore_rendering_finished,
            fence_rendering_finished,
        ) = sync::setup(base)?;

        wait_device_idle(base)?;

        Ok(Self {
            base,
            swap_chain,
            render_pass,
            depth_device_image,
            uniform_buffers,
            descriptor,
            graphics_command,
            transfer_command,
            vertex_device_buffer,
            index_device_buffer,
            graphics_pipeline_layout,
            graphics_pipeline,
            timeline,
            semaphore_image_acquired,
            semaphore_rendering_finished,
            fence_rendering_finished,
            timestamp,
            textures,
        })
    }
}

impl Drop for Setup<'_> {
    fn drop(&mut self) {
        wait_device_idle(self.base).unwrap();

        for (device_image, sampler) in &self.textures {
            device_image.destroy(&self.base);
            sampler.destroy(&self.base);
        }
        self.timestamp.destroy(&self.base);
        self.graphics_command.destroy(&self.base);
        self.transfer_command.destroy(&self.base);
        self.semaphore_image_acquired.destroy(&self.base);
        self.semaphore_rendering_finished.destroy(&self.base);
        self.timeline.destroy(&self.base);
        self.fence_rendering_finished.destroy(&self.base);
        self.vertex_device_buffer.destroy(&self.base);
        self.index_device_buffer.destroy(&self.base);
        self.depth_device_image.destroy(&self.base);
        for uniform_buffer in &self.uniform_buffers {
            uniform_buffer.destroy(&self.base);
        }
        self.swap_chain.destroy(&self.base);
        self.descriptor.destroy(&self.base);
        self.graphics_pipeline_layout.destroy(&self.base);
        self.graphics_pipeline.destroy(&self.base);
        self.render_pass.destroy(&self.base);
    }
}
