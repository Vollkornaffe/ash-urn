// just because you can't iterate over enums
pub const DESCRIPTOR_TYPES: [ash::vk::DescriptorType; 11] = [
    ash::vk::DescriptorType::SAMPLER,
    ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
    ash::vk::DescriptorType::SAMPLED_IMAGE,
    ash::vk::DescriptorType::STORAGE_IMAGE,
    ash::vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
    ash::vk::DescriptorType::STORAGE_TEXEL_BUFFER,
    ash::vk::DescriptorType::UNIFORM_BUFFER,
    ash::vk::DescriptorType::STORAGE_BUFFER,
    ash::vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
    ash::vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
    ash::vk::DescriptorType::INPUT_ATTACHMENT,
];
