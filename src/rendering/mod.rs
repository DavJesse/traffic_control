pub mod draw;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub fn init(title: &str, width: u32, height: u32) -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();

    (sdl_context, canvas)
}

pub use draw::*;