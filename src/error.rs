#[allow(dead_code)]
#[derive(Debug)]
pub enum UrnError {
    GenericDynamic(String),
    Generic(&'static str),
    AshLoadingError(ash::LoadingError),
    AshError(ash::vk::Result),
    AshInstanceError(ash::InstanceError),
    IO(std::io::Error),
    NulError(std::ffi::NulError),
    ImageError(image::error::ImageError),
}

impl From<std::ffi::NulError> for UrnError {
    fn from(e: std::ffi::NulError) -> UrnError {
        UrnError::NulError(e)
    }
}

impl From<std::io::Error> for UrnError {
    fn from(e: std::io::Error) -> UrnError {
        UrnError::IO(e)
    }
}

impl From<ash::vk::Result> for UrnError {
    fn from(e: ash::vk::Result) -> UrnError {
        UrnError::AshError(e)
    }
}

impl From<ash::InstanceError> for UrnError {
    fn from(e: ash::InstanceError) -> UrnError {
        UrnError::AshInstanceError(e)
    }
}

impl From<ash::LoadingError> for UrnError {
    fn from(e: ash::LoadingError) -> UrnError {
        UrnError::AshLoadingError(e)
    }
}

impl From<image::error::ImageError> for UrnError {
    fn from(e: image::error::ImageError) -> UrnError {
        UrnError::ImageError(e)
    }
}
