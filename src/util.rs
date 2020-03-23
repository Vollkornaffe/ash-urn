pub use std::ffi::CString;

/// Alleviating `str` to `*const i8` conversion.
pub struct StringContainer {
    pub buffer: Vec<CString>,
    pub pointer: Vec<*const i8>,
}

pub fn vk_to_string(raw_string_array: &[i8]) -> String {
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        std::ffi::CStr::from_ptr(pointer)
    };
    raw_string
        .to_str()
        .expect("Failed to convert vulkan raw string.")
        .to_owned()
}

impl StringContainer {
    /// Creates buffer & pointer
    pub fn new(strs: Vec<String>) -> Self {
        let buffer: Vec<CString> = strs
            .iter()
            .map(|s| CString::new(s.clone()))
            .collect::<Result<_, _>>()
            .unwrap();
        let pointer: Vec<*const i8> = buffer.iter().map(|cs| cs.as_ptr()).collect();
        Self { buffer, pointer }
    }
}
