use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    // Clear screen to black background
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
    canvas.clear();

    // Set road color
    let road_color = Color::RGB(128, 128, 128); // Gray color for the road
    canvas.set_draw_color(road_color);

    // Draw vertical road
    let vertical_road = Rect::new(350, 0, 100, 600);
    canvas.fill_rect(vertical_road).unwrap();
    
    
    // Draw horizontal road
    let horizontal_road = Rect::new(0, 250, 800, 100);
    canvas.fill_rect(horizontal_road).unwrap();

    // Present drawings to canvas
    canvas.present();
}
