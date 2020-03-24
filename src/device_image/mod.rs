pub mod image;
pub mod memory;
pub mod view;

pub use image::Image;
pub use memory::Memory;
pub use view::View;

pub struct DeviceImage {
    pub image: Image,
    pub memory: Memory,
    pub view: View,
}
