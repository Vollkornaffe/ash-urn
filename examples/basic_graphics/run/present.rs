use crate::AppError;

use ash_urn::Command;
use ash_urn::Semaphore;
use ash_urn::SwapChain;

pub fn submit(
    swap_chain: &SwapChain,
    graphics_command: &Command,
    semaphore_rendering_finished: &Semaphore,
    image_index: u32,
) -> Result<(), AppError> {

    let present_wait_semaphores = [semaphore_rendering_finished.0];
    let swap_chains = [swap_chain.handle];
    let image_indices = [image_index];
    let present_info = ash::vk::PresentInfoKHR::builder()
        .wait_semaphores(&present_wait_semaphores)
        .swapchains(&swap_chains)
        .image_indices(&image_indices);
    unsafe {
        swap_chain
            .loader
            .0
            .queue_present(graphics_command.queue.0, &present_info)?;
    }

    Ok(())

}
