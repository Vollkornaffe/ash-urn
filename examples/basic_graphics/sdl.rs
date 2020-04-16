#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fermium::*;

const SDL_WINDOWPOS_CENTERED: c_int = SDL_WINDOWPOS_CENTERED_MASK as c_int;

use ash::version::InstanceV1_0;
use ash::vk::Handle;

fn c_char_ptr_to_string(mut c_char_ptr: *const c_char) -> String {
    let mut string = String::new();
    unsafe {
        while *c_char_ptr != 0 {
            string.push(*c_char_ptr as u8 as char);
            c_char_ptr = c_char_ptr.offset(1);
        }
    }
    string
}

#[derive(Debug)]
pub struct SdlError(pub String);

impl SdlError {
    pub fn new() -> Self {
        Self(c_char_ptr_to_string(unsafe { SDL_GetError() }))
    }
}

pub enum SdlEvent {
    Close,
    //IncDebug,
    //DecDebug,
    //Resize,
    //Profile,
    //Step,
}

pub struct SDL {
    window: *mut SDL_Window,
    surface: *mut VkSurfaceKHR, // initially is null
}

pub struct WindowSettings {
    pub title: &'static str,
    pub w: u32,
    pub h: u32,
    pub maximized: bool,
}

impl SDL {
    fn create_window(settings: WindowSettings) -> Result<*mut SDL_Window, SdlError> {
        let window_flags = SDL_WINDOW_SHOWN
            | SDL_WINDOW_RESIZABLE
            | SDL_WINDOW_VULKAN
            | if settings.maximized {
                SDL_WINDOW_MAXIMIZED
            } else {
                0
            };

        let window = unsafe {
            SDL_CreateWindow(
                settings.title.as_ptr() as *const c_char,
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                settings.w as i32,
                settings.h as i32,
                window_flags as u32,
            )
        };

        if window.is_null() {
            Err(SdlError::new())
        } else {
            Ok(window)
        }
    }

    pub fn new(settings: WindowSettings) -> Result<SDL, SdlError> {
        if unsafe { SDL_Init(SDL_INIT_VIDEO) } != 0 {
            return Err(SdlError::new());
        }

        let window = Self::create_window(settings)?;

        Ok(Self {
            window,
            surface: std::ptr::null_mut(),
        })
    }

    pub fn required_extension_names(&self) -> Result<Vec<String>, SdlError> {
        // first get count
        let mut count: c_uint = 0;
        if unsafe {
            SDL_Vulkan_GetInstanceExtensions(
                self.window,
                &mut count as *mut c_uint,
                std::ptr::null_mut(),
            )
        } == SDL_FALSE
        {
            return Err(SdlError::new());
        }

        // prepare vec
        let mut extensions = Vec::new();
        extensions.resize(count as usize, std::ptr::null());

        // get the extensions
        if unsafe {
            SDL_Vulkan_GetInstanceExtensions(
                self.window,
                &mut count as *mut c_uint,
                extensions.as_mut_ptr() as *mut *const c_char,
            )
        } == SDL_FALSE
        {
            return Err(SdlError::new());
        }

        Ok(extensions
            .iter()
            .map(|c_char_ptr| c_char_ptr_to_string(*c_char_ptr))
            .collect())
    }

    pub fn create_surface(
        &mut self,
        ash_instance: &ash::Instance,
    ) -> Result<ash::vk::SurfaceKHR, SdlError> {
        let raw_instance = ash_instance.handle().as_raw();
        let mut surface: VkSurfaceKHR = std::ptr::null_mut();
        if unsafe {
            SDL_Vulkan_CreateSurface(
                self.window,
                raw_instance as VkInstance,
                &mut surface as *mut VkSurfaceKHR,
            )
        } == SDL_FALSE
        {
            return Err(SdlError::new());
        }

        self.surface = &mut surface as *mut VkSurfaceKHR;

        Ok(ash::vk::SurfaceKHR::from_raw(surface as u64))
    }

    pub fn get_events(&mut self) -> Vec<SdlEvent> {
        let mut res = Vec::new();
        let mut event = SDL_Event::default();
        unsafe {
            while SDL_PollEvent(&mut event) != 0 {
                match event.type_ as SDL_EventType {
                    SDL_QUIT => {
                        println!("sdl detected close");
                        res.push(SdlEvent::Close);
                    }
                    _ => (),
                }
            }
        }
        res
    }

    pub fn get_size(&self) -> (u32, u32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            SDL_Vulkan_GetDrawableSize(self.window, &mut w as *mut c_int, &mut h as *mut c_int);
        }
        (w as u32, h as u32)
    }

    pub fn destroy(&self) {
        unsafe {
            SDL_DestroyWindow(self.window);
            SDL_Quit();
        }
    }
}
