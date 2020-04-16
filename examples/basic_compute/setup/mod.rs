pub mod base;

mod command;
mod descriptor;
mod mesh_buffers;
mod pipeline;
mod storage_buffers;
mod swap_chain;
mod sync;
mod textures;
mod uniform_buffers;

use crate::AppError;
use crate::ComputeUBO;
use crate::Particles;
use crate::SDL;

use ash_urn::sync::wait_device_idle;
use ash_urn::Base;
use ash_urn::Command;
use ash_urn::ComputePipeline;
use ash_urn::Descriptor;
use ash_urn::DeviceBuffer;
use ash_urn::DeviceImage;
use ash_urn::Fence;
use ash_urn::GraphicsPipeline;
use ash_urn::Mesh;
use ash_urn::PipelineLayout;
use ash_urn::RenderPass;
use ash_urn::Sampler;
use ash_urn::Semaphore;
use ash_urn::SwapChain;
use ash_urn::Timeline;
use ash_urn::Timestamp;

pub struct Setup<'a> {
    pub base: &'a Base,
    pub swap_chain: SwapChain,
    pub render_pass: RenderPass,
    pub depth_device_image: DeviceImage,

    pub graphics_uniform_buffers: Vec<DeviceBuffer>,
    pub compute_uniform_buffer: DeviceBuffer,

    pub graphics_descriptor: Descriptor,
    pub compute_descriptor: Descriptor,

    pub graphics_command: Command,
    pub compute_command: Command,
    pub transfer_command: Command,

    pub vertex_device_buffer: DeviceBuffer,
    pub index_device_buffer: DeviceBuffer,

    pub reference_buffer: DeviceBuffer,
    pub particle_buffer: DeviceBuffer,

    pub graphics_pipeline_layout: PipelineLayout,
    pub graphics_pipeline: GraphicsPipeline,

    pub compute_pipeline_layout: PipelineLayout,
    pub calculate_pipeline: ComputePipeline,
    pub integrate_pipeline: ComputePipeline,

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
        reference_mesh: &Mesh,
        particles: &Particles,
    ) -> Result<Self, AppError> {
        wait_device_idle(base)?;

        // replicate reference mesh for each particle
        let mesh = particles.as_mesh(reference_mesh, 0.1);

        // get swap chain + renderpass & depth image
        // this is also a bit entangled
        let (swap_chain, render_pass, depth_device_image) =
            swap_chain::setup(base, &sdl, &surface_loader, surface)?;

        // an uniform buffer per swapchain image
        let graphics_uniform_buffers =
            uniform_buffers::setup_graphics(base, swap_chain.image_count)?;
        let compute_uniform_buffer = uniform_buffers::setup_compute(base)?;
        // write to the compute ubo
        compute_uniform_buffer.write(
            &base,
            &ComputeUBO {
                n_particles: particles.0.len() as u32,
                n_reference: reference_mesh.vertices.len() as u32,
                scale: 0.01,
                d_t: 0.0001,
                G: 0.01,
            },
        )?;

        // get the structures for commands,
        // they will be filled out later
        let (graphics_command, compute_command, transfer_command) =
            command::setup(base, swap_chain.image_count)?;

        // create device buffers from the mesh & load the textures
        // the transfer is done with the transfer command,
        // ownership is transferred afterwards
        let (vertex_device_buffer, index_device_buffer) =
            mesh_buffers::setup(base, &mesh, &graphics_command, &transfer_command)?;
        let textures = textures::setup(
            base,
            &[(
                "examples/basic_compute/assets/rainbow.jpg".to_string(),
                "MuskyBoy".to_string(),
            )],
            &graphics_command,
            &transfer_command,
        )?;

        // prepare storage buffers
        let (reference_buffer, particle_buffer) = storage_buffers::setup(
            base,
            reference_mesh,
            particles,
            &compute_command,
            &transfer_command,
        )?;

        // these sets contain the respective UBOs & combined image samplers
        let graphics_descriptor =
            descriptor::setup_graphics(base, &graphics_uniform_buffers, &textures[0])?;

        // this set contains the compute UBO & storage buffers
        let compute_descriptor = descriptor::setup_compute(
            base,
            &compute_uniform_buffer,
            &reference_buffer,
            &particle_buffer,
            &vertex_device_buffer,
        )?;

        // just one pipeline, using the vert & frag shader
        let (graphics_pipeline_layout, graphics_pipeline) =
            pipeline::setup_graphics(base, &graphics_descriptor, &swap_chain, &render_pass)?;

        // and just one pipeline for the particle update
        let (compute_pipeline_layout, calculate_pipeline, integrate_pipeline) =
            pipeline::setup_compute(base, &compute_descriptor)?;

        // get timestamp for profiling
        let timestamp = Timestamp::new(
            &base,
            [
                "CALCULATE_START",
                "CALCULATE_DONE",
                "INTEGRATE_START",
                "INTEGRATE_DONE",
                "RENDER_START",
                "RENDER_DONE",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            base.physical_device.timestamp_period(&base.instance.0)?,
            "Timestamp".to_string(),
        )?;

        // write to the graphics command buffers
        command::write_graphics(
            base,
            &graphics_command,
            &timestamp,
            &render_pass,
            &swap_chain,
            &graphics_pipeline,
            &graphics_pipeline_layout,
            &graphics_descriptor,
            &vertex_device_buffer,
            &index_device_buffer,
            &mesh,
        )?;

        // write to the one compute buffer
        command::write_compute(
            base,
            &timestamp,
            &compute_pipeline_layout,
            &calculate_pipeline,
            &integrate_pipeline,
            &compute_command,
            &compute_descriptor,
            particles.0.len() as u32,
        )?;

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
            graphics_uniform_buffers,
            compute_uniform_buffer,
            graphics_descriptor,
            compute_descriptor,
            graphics_command,
            compute_command,
            transfer_command,
            vertex_device_buffer,
            index_device_buffer,
            reference_buffer,
            particle_buffer,
            graphics_pipeline_layout,
            graphics_pipeline,
            compute_pipeline_layout,
            calculate_pipeline,
            integrate_pipeline,
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
        self.compute_command.destroy(&self.base);
        self.transfer_command.destroy(&self.base);
        self.semaphore_image_acquired.destroy(&self.base);
        self.semaphore_rendering_finished.destroy(&self.base);
        self.timeline.destroy(&self.base);
        self.fence_rendering_finished.destroy(&self.base);
        self.vertex_device_buffer.destroy(&self.base);
        self.index_device_buffer.destroy(&self.base);
        self.reference_buffer.destroy(&self.base);
        self.particle_buffer.destroy(&self.base);
        self.depth_device_image.destroy(&self.base);
        for uniform_buffer in &self.graphics_uniform_buffers {
            uniform_buffer.destroy(&self.base);
        }
        self.compute_uniform_buffer.destroy(&self.base);
        self.swap_chain.destroy(&self.base);
        self.graphics_descriptor.destroy(&self.base);
        self.compute_descriptor.destroy(&self.base);
        self.graphics_pipeline_layout.destroy(&self.base);
        self.graphics_pipeline.destroy(&self.base);
        self.compute_pipeline_layout.destroy(&self.base);
        self.calculate_pipeline.destroy(&self.base);
        self.integrate_pipeline.destroy(&self.base);
        self.render_pass.destroy(&self.base);
    }
}
