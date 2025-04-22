use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    // Clear screen to black background
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
    canvas.clear();                             // Uses background color(black) to clear canvas

    // Set road color
    let road_color = Color::RGB(128, 128, 128); // Gray color for the road
    canvas.set_draw_color(road_color);

    // Draw vertical road
    let vertical_road = Rect::new(350, 0, 100, 600);
    canvas.fill_rect(vertical_road).unwrap();

    // Draw top lane separator
    let top_lane_separator = Rect::new(398, 0, 4, 250);
    canvas.fill_rect(top_lane_separator).unwrap();

     // Draw bottom lane separator
     let bottom_lane_separator = Rect::new(398, 350, 4, 250);
     canvas.fill_rect(bottom_lane_separator).unwrap();
    
    
    // Draw horizontal road
    let horizontal_road = Rect::new(0, 250, 800, 100);
    canvas.fill_rect(horizontal_road).unwrap();

    // Draw left lane separator
    let left_lane_separator = Rect::new(0, 298, 350, 4);
    canvas.fill_rect(left_lane_separator).unwrap();

     // Draw bottom lane separator
     let right_lane_separator = Rect::new(450, 298, 350, 4);
     canvas.fill_rect(right_lane_separator).unwrap();

    // Present drawings to canvas
    canvas.present();
}
