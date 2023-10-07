use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

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

    let sprites = load_media(&texture_creator)?;
    let mut current_image: &str = "press";

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'app;
            }
        }

        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::Up) {
            current_image = "up";
        }
        if keyboard_state.is_scancode_pressed(Scancode::Down) {
            current_image = "down";
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            current_image = "right";
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            current_image = "left";
        }

        // Clear and render the currently selected image
        canvas.clear();
        canvas.copy(&sprites[current_image], None, None)?;

        canvas.present();
    }

    Ok(())
}

fn load_media(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<HashMap<&'static str, Texture>, String> {
    let mut map = HashMap::new();
    map.insert("up", texture_creator.load_texture("resources/up.bmp")?);
    map.insert("down", texture_creator.load_texture("resources/down.bmp")?);
    map.insert("left", texture_creator.load_texture("resources/left.bmp")?);
    map.insert(
        "right",
        texture_creator.load_texture("resources/right.bmp")?,
    );
    map.insert(
        "press",
        texture_creator.load_texture("resources/press.bmp")?,
    );
    Ok(map)
}
