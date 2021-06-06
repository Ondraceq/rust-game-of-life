pub mod callback_handler;
pub mod size;

pub use callback_handler::CallbackHandler;
pub use size::WindowSize;

pub trait IntoWindowError: Into<Box<dyn std::error::Error>> {}

impl IntoWindowError for String {}
impl IntoWindowError for sdl2::video::WindowBuildError {}
impl IntoWindowError for sdl2::IntegerOrSdlError {}

#[derive(Debug)]
pub enum Error {
    Quit,
    Config(String),
    Err(Box<dyn std::error::Error>),
}

impl<T> From<T> for self::Error
where
    T: IntoWindowError,
{
    fn from(e: T) -> Self {
        Self::Err(e.into())
    }
}

impl std::fmt::Display for self::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quit => write!(f, "Window quit"),
            Self::Config(s) => write!(f, "Window config error ({})", s),
            Self::Err(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for self::Error {}

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug)]
pub struct WindowConfig {
    pub window_name: String,
    pub size: WindowSize,
    pub update_frame_duration: std::time::Duration,
    pub background_color: sdl2::pixels::Color,
}

pub fn fps_to_duration(fps: u8) -> Result<std::time::Duration> {
    assert!(fps > 0);
    if fps == 0 {
        Err(Error::Config("FPS set to 0".into()))
    } else {
        Ok(std::time::Duration::new(
            0,
            1_000_000_000u32 / u32::from(fps),
        ))
    }
}

impl WindowConfig {
    #[allow(dead_code)]
    pub fn set_fps(&mut self, new_fps: u8) -> Result<()> {
        assert!(new_fps > 0);
        self.update_frame_duration = fps_to_duration(new_fps)?;
        Ok(())
    }
}

pub fn sleep_until(instant: std::time::Instant) {
    let now = std::time::Instant::now();
    if instant > now {
        std::thread::sleep(instant - now);
    }
}

pub struct WindowData {
    pub window_config: WindowConfig,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub mouse_pos: (i32, i32),
}

fn build(window_config: WindowConfig) -> Result<(WindowData, sdl2::EventPump)> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            &window_config.window_name,
            window_config.size.get_width(),
            window_config.size.get_height(),
        )
        .position_centered()
        .build()?;
    let canvas = window.into_canvas().build()?;
    let event_pump = sdl_context.event_pump()?;
    let window_data = WindowData {
        window_config,
        canvas,
        mouse_pos: (0, 0),
    };
    Ok((window_data, event_pump))
}

pub fn run(window_config: WindowConfig, mut handler: CallbackHandler<WindowData>) -> Result<()> {
    let (mut window_data, mut event_pump) = build(window_config)?;

    loop {
        let next_cycle_time =
            std::time::Instant::now() + window_data.window_config.update_frame_duration;

        // Clear canvas
        window_data
            .canvas
            .set_draw_color(window_data.window_config.background_color);
        window_data.canvas.clear();

        // Handle events
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::MouseMotion { x, y, .. } = event {
                window_data.mouse_pos = (x, y);
            }
            handler.call_on_event_cbs(&mut window_data, &event)?;
        }

        // On frame callbacks
        handler.call_on_frame_cbs(&mut window_data)?;
        window_data.canvas.present();

        // Sleep for the rest of the cycle
        sleep_until(next_cycle_time);
    }
}
