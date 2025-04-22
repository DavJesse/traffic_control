mod rendering;

use rendering::{draw_roads, init};
use std::time::Duration;

fn main() {
    let (sdl_context, mut canvas) = init("Traffic Simulator", 800, 600);
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        draw_roads(&mut canvas);
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}
