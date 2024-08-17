use sdl2::{render::WindowCanvas, EventPump, Sdl, VideoSubsystem};

pub fn create_window(width: u32, hegiht: u32) -> (Sdl, VideoSubsystem, WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Asteroid", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();
    let window_canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    (sdl_context, video_subsystem, window_canvas, event_pump)
}