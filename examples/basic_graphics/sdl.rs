#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fermium::*;

const SDL_WINDOWPOS_CENTERED: c_int = SDL_WINDOWPOS_CENTERED_MASK as c_int;

use ash::version::InstanceV1_0;
use ash::vk::Handle;

#[derive(Debug)]
pub struct SdlError(pub String);

impl SdlError {
    pub fn new() -> Self {
        unsafe {
            let mut c_char_ptr: *const c_char = SDL_GetError();
            let mut error_msg = String::new();
            while *c_char_ptr != 0 {
                error_msg.push(*c_char_ptr as u8 as char);
                c_char_ptr = c_char_ptr.offset(1);
            }
            Self(error_msg)
        }
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
}

pub struct WindowSettings {
    pub title: &'static str,
    pub w: u32,
    pub h: u32,
    pub maximized: bool,
}

impl SDL {

    fn create_window(
        settings: WindowSettings,
    ) -> Result<*mut SDL_Window, SdlError> {

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
        })
    }

    /*
    pub fn required_extension_names(&self) -> Result<Vec<String>, SdlError> {
        Ok(self
            .window
            .vulkan_instance_extensions()?
            .iter()
            .map(|s| s.to_string())
            .collect())
    }

    pub fn create_surface(
        &mut self,
        ash_instance: &ash::Instance,
    ) -> Result<ash::vk::SurfaceKHR, SdlError> {
        let raw_instance = ash_instance.handle().as_raw() as usize;
        let surface = self.window.vulkan_create_surface(raw_instance)?;
        let ash_surface = ash::vk::SurfaceKHR::from_raw(surface);

        self.surface = Some(surface);

        Ok(ash_surface)
    }

    pub fn get_events(&mut self) -> Vec<SdlEvent> {
        let mut res = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    println!("sdl detected close");
                    res.push(SdlEvent::Close);
                }
                _ => {}
            }
        }
        res
    }
    */
}
