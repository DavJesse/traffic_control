pub mod draw;    // Declare draw.rs as part of the 'rendering' module
pub use draw::*; // Export all public items from draw.rs

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

// Initialize SDL2 and create a window and canvas
pub fn init(title: &str, width: u32, height: u32) -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();            // Initialize SDL2system
    let video_subsystem = sdl_context.video().unwrap(); // Get the video subsystem

    // Create a window with the specified title and dimensions
    let window = video_subsystem
        .window(title, width, height)
        .position_centered() // Center the window on the screen
        .build()             //Builds window using builder pattern
        .unwrap();

    // Create a canvas from the window
    let canvas = window.into_canvas().build().unwrap();

    (sdl_context, canvas)
}
