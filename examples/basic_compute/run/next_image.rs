use crate::AppError;

use ash_urn::Semaphore;
use ash_urn::SwapChain;

pub fn aquire(
    swap_chain: &SwapChain,
    semaphore_image_acquired: &Semaphore,
) -> Result<u32, AppError> {
    let (image_index, _suboptimal) = unsafe {
        swap_chain.loader.0.acquire_next_image(
            swap_chain.handle,
            std::u64::MAX,
            semaphore_image_acquired.0,
            ash::vk::Fence::default(),
        )?
    };

    Ok(image_index)
}
