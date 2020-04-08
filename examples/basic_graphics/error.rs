use crate::sdl;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    GenericDynamic(String),
    Generic(&'static str),
    UrnError(ash_urn::UrnError),
    AshError(ash::vk::Result),
    AshInstanceError(ash::InstanceError),
    IO(std::io::Error),
    NulError(std::ffi::NulError),
    SdlError(sdl::SdlError),
}

impl From<std::ffi::NulError> for AppError {
    fn from(e: std::ffi::NulError) -> AppError {
        AppError::NulError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IO(e)
    }
}

impl From<ash::vk::Result> for AppError {
    fn from(e: ash::vk::Result) -> AppError {
        AppError::AshError(e)
    }
}

impl From<ash::InstanceError> for AppError {
    fn from(e: ash::InstanceError) -> AppError {
        AppError::AshInstanceError(e)
    }
}

impl From<ash_urn::UrnError> for AppError {
    fn from(e: ash_urn::UrnError) -> AppError {
        AppError::UrnError(e)
    }
}

impl From<sdl::SdlError> for AppError {
    fn from(e: sdl::SdlError) -> AppError {
        AppError::SdlError(e)
    }
}
