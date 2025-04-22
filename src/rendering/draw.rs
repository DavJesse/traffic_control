use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    let width = canvas.output_size().unwrap().0 as i32;
    let height = canvas.output_size().unwrap().1 as i32;

    canvas.set_draw_color(Color::RGB(30, 30, 30)); // road color
    canvas.clear();

    let road_width = 100;

    // Draw vertical road
    canvas.fill_rect(Rect::new((width - road_width) / 2, 0, road_width as u32, height as u32)).unwrap();

    // Draw horizontal road
    canvas.fill_rect(Rect::new(0, (height - road_width) / 2, width as u32, road_width as u32)).unwrap();

    canvas.present();
}
