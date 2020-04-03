mod sdl;

use ash_urn::base::{
    Base, Entry, Instance, InstanceSettings, LogicalDevice, LogicalDeviceSettings, PhysicalDevice,
    PhysicalDeviceSettings, Validation,
};

use ash_urn::descriptor;
use ash_urn::device_image::create_depth_device_image;
use ash_urn::transfer::{create_index_device_buffer, create_vertex_device_buffer, ownership};
use ash_urn::{command, Command, CommandSettings};
use ash_urn::{Descriptor, DescriptorSettings};
use ash_urn::{DeviceBuffer, DeviceBufferSettings};
use ash_urn::{GraphicsPipeline, GraphicsPipelineSettings};
use ash_urn::{Indices, Mesh, Vertex};
use ash_urn::{PipelineLayout, PipelineLayoutSettings};
use ash_urn::{RenderPass, RenderPassSettings};
use ash_urn::{SwapChain, SwapChainSettings};

use ash::version::DeviceV1_0;

const ENABLE_VALIDATION: bool = cfg!(debug_assertions);

use ash_urn::memory_alignment::Align16;

use std::collections::HashMap;

#[repr(C)]
struct UBO {
    model: Align16<cgmath::Matrix4<f32>>,
    view: Align16<cgmath::Matrix4<f32>>,
    proj: Align16<cgmath::Matrix4<f32>>,
}

fn main() {
    // create a mesh to render
    let mesh = Mesh::new().add_quad(
        [-1.0, -1.0, 0.0],
        [1.0, -1.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 1.0],
    );

    // first of all create sdl context
    let mut sdl = sdl::SDL::new(sdl::WindowSettings {
        title: "Test",
        w: 800,
        h: 800,
        maximized: false,
    })
    .unwrap();

    // Get our requriements ready
    let mut instance_extension_names = sdl.required_extension_names().unwrap();
    instance_extension_names.push(
        ash::extensions::ext::DebugUtils::name()
            .to_str()
            .unwrap()
            .to_string(),
    );
    instance_extension_names.push("VK_KHR_get_physical_device_properties2".to_string());
    let validation_layer_names = vec!["VK_LAYER_KHRONOS_validation".to_string()];

    let entry = Entry::new().unwrap();
    // Instance needs vulkan version
    let instance = Instance::new(
        InstanceSettings {
            name: "Test".to_string(),
            version_major: 1,
            version_minor: 2,
            version_patch: 131,
            extension_names: instance_extension_names,
            enable_validation: ENABLE_VALIDATION,
            validation_layer_names: validation_layer_names.clone(),
        },
        &entry.0,
    )
    .unwrap();
    // Get our input validated!
    let validation = if ENABLE_VALIDATION {
        Some(Validation::new(&entry.0, &instance.0).unwrap())
    } else {
        None
    };
    // Ready for the surface to draw on
    let surface_loader = ash::extensions::khr::Surface::new(&entry.0, &instance.0);
    let surface = sdl.create_surface(&instance.0).unwrap();

    // Time to think about devices
    let timelines = false;
    let mut device_extensions = vec!["VK_KHR_swapchain".to_string()];
    if timelines {
        device_extensions.push("VK_KHR_timeline_semaphore".to_string());
    }
    // First get the actual gpu
    let physical_device = PhysicalDevice::pick_gpu(
        &instance.0,
        device_extensions.clone(),
        &surface_loader,
        surface,
        PhysicalDeviceSettings {
            timelines,
            subgroups: true,
        },
    )
    .unwrap();

    let queue_map = physical_device
        .query_queues(&instance.0, &surface_loader, surface)
        .unwrap();
    let transfer_queue_family_idx = queue_map
        .get(&ash_urn::base::queue_families::DEDICATED_TRANSFER)
        .unwrap()
        .idx;
    let combined_queue_family_idx = queue_map
        .get(&ash_urn::base::queue_families::COMBINED)
        .unwrap()
        .idx;

    // Then the logical device that does all of the heavy lifting
    let logical_device = LogicalDevice::new(
        &instance.0,
        physical_device.0,
        LogicalDeviceSettings {
            extension_names: device_extensions,
            enable_validation: ENABLE_VALIDATION,
            validation_layer_names: validation_layer_names.clone(),
            queues: vec![transfer_queue_family_idx, combined_queue_family_idx],
            timelines,
        },
    )
    .unwrap();

    // Combine everything into the Base
    let base = Base {
        entry,
        instance,
        validation,
        physical_device,
        logical_device,
    };

    // Create swapchain
    let swap_chain_support = base
        .physical_device
        .query_swap_chain_support(&surface_loader, surface)
        .unwrap();
    let mut swap_chain = SwapChain::new(
        &base,
        &SwapChainSettings {
            w: sdl.window.size().0,
            h: sdl.window.size().1,
            support: swap_chain_support,
            surface: surface,
            image_count: 2,
            name: "SwapChain".to_string(),
        },
    )
    .unwrap();

    // Create render pass
    let render_pass = RenderPass::new(
        &base,
        &RenderPassSettings {
            swap_chain_format: swap_chain.surface_format.0.format,
            name: "RenderPass".to_string(),
        },
    )
    .unwrap();

    // create one depth image
    let depth_device_image = create_depth_device_image(&base, swap_chain.extent.0).unwrap();

    // now we can fill out the swapchain elements
    swap_chain
        .fill_elements(&base, depth_device_image.view.0, render_pass.0)
        .unwrap();

    // UBO for each fram in flight
    let mut uniform_buffers = Vec::new();
    for i in 0..swap_chain.image_count {
        uniform_buffers.push(
            DeviceBuffer::new(
                &base,
                &DeviceBufferSettings {
                    size: std::mem::size_of::<UBO>() as ash::vk::DeviceSize,
                    usage: ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                    properties: ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                        | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
                    name: format!("UniformBuffer_{}", i),
                },
            )
            .unwrap(),
        );
    }

    // create descriptor sets
    let descriptor = {
        let mut setup_map = HashMap::new();
        setup_map.insert(
            0,
            descriptor::Setup {
                ty: ash::vk::DescriptorType::UNIFORM_BUFFER,
                stage: ash::vk::ShaderStageFlags::VERTEX,
            },
        );
        let mut set_usages = Vec::new();
        for (i, uniform_buffer) in uniform_buffers.iter().enumerate() {
            let mut usages = HashMap::new();
            usages.insert(0, descriptor::Usage::Buffer(uniform_buffer.buffer.0));
            set_usages.push(descriptor::SetUsage {
                usages,
                name: format!("DescriptorSet_{}", i),
            });
        }
        Descriptor::new(
            &base,
            &DescriptorSettings {
                setup_map,
                set_usages,
                name: "Descriptor".to_string(),
            },
        )
        .unwrap()
    };

    // Create graphic commands, one buffer per image
    let graphics_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: combined_queue_family_idx,
            n_buffer: swap_chain.image_count,
            name: "GraphicsCommand".to_string(),
        },
    )
    .unwrap();

    // Just need the queue for presenting
    let present_queue =
        command::Queue::new(&base, combined_queue_family_idx, "PresentQueue".to_string()).unwrap();

    // Transfer no buffers allocated, because one-time commands only
    let transfer_command = Command::new(
        &base,
        &CommandSettings {
            queue_family_idx: transfer_queue_family_idx,
            n_buffer: 0,
            name: "TransferCommand".to_string(),
        },
    )
    .unwrap();

    // create vertex buffer
    let vertex_device_buffer = create_vertex_device_buffer(
        &base,
        mesh.vertices.as_slice(),
        transfer_command.queue.0,
        transfer_command.pool.0,
        "VertexBuffer".to_string(),
    )
    .unwrap();

    // create index buffer
    let index_device_buffer = create_index_device_buffer(
        &base,
        mesh.indices.as_slice(),
        transfer_command.queue.0,
        transfer_command.pool.0,
        "IndexBuffer".to_string(),
    )
    .unwrap();

    // transfer the ownership to the combined queue family
    ownership::transfer_to_combined(
        &base,
        &[&vertex_device_buffer, &index_device_buffer],
        &[],
        &transfer_command,
        &graphics_command, // any command struct from the combined family is ok
    )
    .unwrap();

    // Create a single graphics pipeline
    let graphics_pipeline_layout = PipelineLayout::new(
        &base,
        &PipelineLayoutSettings {
            set_layouts: vec![descriptor.layout.0],
            push_constant_ranges: vec![],
            name: "GraphicsPipelineLayout".to_string(),
        },
    )
    .unwrap();
    let graphics_pipeline = GraphicsPipeline::new(
        &base,
        &GraphicsPipelineSettings {
            layout: graphics_pipeline_layout.0,
            vert_spv: "examples/shaders/vert.spv".to_string(),
            frag_spv: "examples/shaders/frag.spv".to_string(),
            extent: swap_chain.extent.0,
            render_pass: render_pass.0,
            name: "GraphicsPipeline".to_string(),
        },
    )
    .unwrap();

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

    'running: loop {
        for e in sdl.get_events() {
            match e {
                sdl::SdlEvent::Close => break 'running,
                _ => {}
            }
        }
    }

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
