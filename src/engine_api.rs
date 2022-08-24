extern crate sdl2;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::Window;

 

pub struct App {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub events: EventPump,
    pub window: Window
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> App {
        let sdl_context = match sdl2::init() {
            Ok(sdl) => sdl,
            Err(err) => panic!("{}", err),
        };
        let video_subsystem = match sdl_context.video() {
            Ok(video) => video,
            Err(err) => panic!("{}", err),
        };
        let events = match sdl_context.event_pump() {
            Ok(event_pump) => event_pump,
            Err(err) => panic!("{}", err),
        };
        let window = match video_subsystem.window(title, width, height).resizable().build() {
            Ok(window) => window,
            Err(err) => panic!("{}", err),
        };
        Self {
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            events: events,
            window: window
        }
    }
}