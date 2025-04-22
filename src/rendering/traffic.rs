use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy)]
pub enum TrafficLightState {
    Red,
    Green,
}

pub fn draw_traffic_light( canvas: &mut Canvas<Window>, x: i32,
    y: i32, width: u32, height: u32, state: TrafficLightState,
) {
    let color = match state {
        TrafficLightState::Red => Color::RGB(255, 0, 0),
        TrafficLightState::Green => Color::RGB(0, 255, 0),
    };

    canvas.set_draw_color(color);
    let rect = Rect::new(x, y, width, height);
    canvas.fill_rect(rect).unwrap();
}
