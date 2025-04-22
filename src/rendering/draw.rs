use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    // Clear screen to black background
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
    canvas.clear();                             // Uses background color(black) to clear canvas

    // Set road color
    let road_color = Color::RGB(82, 94, 106); // Gray color for the road
    canvas.set_draw_color(road_color);

    // Draw vertical road
    let vertical_road = Rect::new(350, 0, 100, 600);
    canvas.fill_rect(vertical_road).unwrap();

    // Draw horizontal road
    let horizontal_road = Rect::new(0, 250, 800, 100);
    canvas.fill_rect(horizontal_road).unwrap();

    // Set color for lane separators
    let lane_separator_color = Color::RGB(255, 255, 255);
    canvas.set_draw_color(lane_separator_color);

    // Draw lane separator
    vertical_dotted_line(canvas, 20, 15, 398, 4, 0, 240);     // Draw top lane separator
    vertical_dotted_line(canvas, 20, 15, 398, 4, 350, 600);   // Draw bottom lane separator 
    horizontal_dotted_line(canvas, 4, 15, 20, 0, 350, 298);   // Draw left lane separator
    horizontal_dotted_line(canvas, 4, 15, 20, 450, 800, 298); // Draw bottom lane separator

    // Present drawings to canvas
    canvas.present();
}

fn vertical_dotted_line(canvas: &mut Canvas<Window>, dash_height:u32, gap:i32, x:i32, width:u32, mut y:i32, max_y:i32) {
    while y < max_y {
        let dash = Rect::new(x, y, width, dash_height);
        canvas.fill_rect(dash).unwrap();
        y += dash_height as i32 + gap;
    }
}

fn horizontal_dotted_line(canvas: &mut Canvas<Window>, height:u32, gap:i32, dash_width:u32, mut x:i32, max_x:i32, y:i32) {
    while x < max_x {
        let dash = Rect::new(x, y, dash_width, height);
        canvas.fill_rect(dash).unwrap();
        x += dash_width as i32 + gap;
    }
}