use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

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

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("resources/viewport.png")?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let top_left_viewport = Rect::new(0, 0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    canvas.set_viewport(top_left_viewport);
    canvas.copy(&texture, None, None)?;

    let top_right_viewport = Rect::new(
        (SCREEN_WIDTH / 2) as i32,
        0,
        SCREEN_WIDTH / 2,
        SCREEN_HEIGHT / 2,
    );
    canvas.set_viewport(top_right_viewport);
    canvas.copy(&texture, None, None)?;

    let bottom_viewport = Rect::new(
        0,
        (SCREEN_HEIGHT / 2) as i32,
        SCREEN_WIDTH,
        SCREEN_HEIGHT / 2,
    );
    canvas.set_viewport(bottom_viewport);
    canvas.copy(&texture, None, None)?;

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
