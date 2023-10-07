use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;

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
    let surface = Surface::load_bmp("resources/hello_world.bmp")
        .map_err(|e| format!("error while initializing surface. {e}"))?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .map_err(|e| format!("error while creating texture. {e}"))?;

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
