use crate::AppError;

use ash_urn::Base;
use ash_urn::{Fence, Semaphore, Timeline};

pub fn setup(base: &Base) -> Result<(Timeline, Semaphore, Semaphore, Fence), AppError> {

    let timeline = Timeline::new(&base, 0, "Timeline".to_string())?;
    let semaphore_image_acquired =
        Semaphore::new(&base, "SemaphoreImageAquired".to_string())?;
    let semaphore_rendering_finished =
        Semaphore::new(&base, "SemaphoreRenderingFinished".to_string())?;
    let fence_rendering_finished =
        Fence::new(&base, true, "FenceRenderingFinished".to_string())?;

    Ok((timeline, semaphore_image_acquired, semaphore_rendering_finished, fence_rendering_finished))
}
