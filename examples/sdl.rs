use ash::version::InstanceV1_0;
use ash::vk::Handle;

#[derive(Debug)]
pub enum SdlError {
    Generic(String),
    Window(sdl2::video::WindowBuildError),
}
impl From<String> for SdlError {
    fn from(e: String) -> SdlError {
        SdlError::Generic(e)
    }
}

pub enum SdlEvent {
    Close,
    IncDebug,
    DecDebug,
    Resize,
    Profile,
    Step,
}

pub struct SDL {
    pub context: sdl2::Sdl,
    pub window: sdl2::video::Window,
    surface: Option<sdl2::video::VkSurfaceKHR>, // This needs a vulkan instance
    event_pump: sdl2::EventPump,
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
        context: &mut sdl2::Sdl,
    ) -> Result<sdl2::video::Window, SdlError> {
        let mut window = context
            .video()?
            .window(settings.title, settings.w, settings.h);

        window.vulkan().position_centered().resizable();

        if settings.maximized {
            window.maximized();
        }

        window.build().map_err(SdlError::Window)
    }

    pub fn new(settings: WindowSettings) -> Result<SDL, SdlError> {
        let mut context = sdl2::init()?;
        let window = Self::create_window(settings, &mut context)?;

        let event_pump = context.event_pump()?;

        Ok(Self {
            context,
            window,
            surface: None, // this is going to be filled later
            event_pump,
        })
    }

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
                sdl2::event::Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    sdl2::event::WindowEvent::Resized(_w, _h) => {
                        println!("sdl detected resize");
                        res.push(SdlEvent::Resize);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        res
    }
}
