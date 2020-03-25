use crate::Base;
use crate::UrnError;

pub fn color_description(
    swapchain_format: ash::vk::Format,
) -> ash::vk::AttachmentDescriptionBuilder<'static> {
    ash::vk::AttachmentDescription::builder()
        .format(swapchain_format)
        .samples(ash::vk::SampleCountFlags::TYPE_1)
        .load_op(ash::vk::AttachmentLoadOp::CLEAR)
        .store_op(ash::vk::AttachmentStoreOp::STORE)
        .stencil_load_op(ash::vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(ash::vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(ash::vk::ImageLayout::UNDEFINED)
        .final_layout(ash::vk::ImageLayout::PRESENT_SRC_KHR)
}

pub fn depth_description(
    base: &Base,
) -> Result<ash::vk::AttachmentDescriptionBuilder<'static>, UrnError> {
    let format = base.find_supported_format(
        vec![
            ash::vk::Format::D32_SFLOAT,
            ash::vk::Format::D32_SFLOAT_S8_UINT,
            ash::vk::Format::D24_UNORM_S8_UINT,
        ],
        ash::vk::ImageTiling::OPTIMAL,
        ash::vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )?;

    Ok(ash::vk::AttachmentDescription::builder()
        .format(format)
        .samples(ash::vk::SampleCountFlags::TYPE_1)
        .load_op(ash::vk::AttachmentLoadOp::CLEAR)
        .store_op(ash::vk::AttachmentStoreOp::DONT_CARE)
        .stencil_load_op(ash::vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(ash::vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(ash::vk::ImageLayout::UNDEFINED)
        .final_layout(ash::vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL))
}
