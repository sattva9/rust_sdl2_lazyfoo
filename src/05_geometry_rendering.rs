use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("first window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| format!("error while initializing window. {e}"))?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| format!("error while initializing canvas. {e}"))?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(Rect::new(
        (SCREEN_WIDTH / 4) as i32,
        (SCREEN_HEIGHT / 4) as i32,
        SCREEN_WIDTH / 2,
        SCREEN_HEIGHT / 2,
    ))?;

    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.draw_rect(Rect::new(
        (SCREEN_WIDTH / 6) as i32,
        (SCREEN_HEIGHT / 6) as i32,
        SCREEN_WIDTH * 2 / 3,
        SCREEN_HEIGHT * 2 / 3,
    ))?;

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.draw_line(
        Point::new(0, (SCREEN_HEIGHT / 2) as i32),
        Point::new(SCREEN_WIDTH as i32, (SCREEN_HEIGHT / 2) as i32),
    )?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for i in (0..SCREEN_HEIGHT).step_by(4) {
        canvas.draw_point(Point::new((SCREEN_WIDTH / 2) as i32, i as i32))?;
    }

    canvas.present();

    'app: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'app;
            }
        }
    }

    Ok(())
}
