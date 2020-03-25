pub struct PresentMode(pub ash::vk::PresentModeKHR);

impl PresentMode {
    pub fn choose(
        available_modes: &[ash::vk::PresentModeKHR],
    ) -> Self {
        for &mode in available_modes {
            if mode == ash::vk::PresentModeKHR::IMMEDIATE {
                return Self(mode);
            }
            //if mode == ash::vk::PresentModeKHR::MAILBOX {
            //    return mode;
            //}
        }

        Self(ash::vk::PresentModeKHR::FIFO) // guaranteed to be there
    }
}
