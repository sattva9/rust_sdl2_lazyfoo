use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

#[derive(Eq, PartialEq, Hash)]
enum KeyPress {
    Up,
    Down,
    Left,
    Right,
    Press,
    Quit,
    Invalid,
}

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
    let media = load_media(&texture_creator)?;

    canvas.copy(&media.get(&KeyPress::Press).unwrap(), None, None)?;
    canvas.present();
    'app: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            let key = match event {
                Event::Quit { .. } => KeyPress::Quit,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) | Some(Keycode::Q) | None => KeyPress::Quit,
                    Some(Keycode::Up) => KeyPress::Up,
                    Some(Keycode::Down) => KeyPress::Down,
                    Some(Keycode::Left) => KeyPress::Left,
                    Some(Keycode::Right) => KeyPress::Right,
                    _ => KeyPress::Invalid,
                },
                _ => KeyPress::Invalid,
            };
            if key.eq(&KeyPress::Quit) {
                break 'app;
            }
            if key.eq(&KeyPress::Invalid) {
                continue;
            }

            canvas.copy(media.get(&key).unwrap(), None, None)?;
            canvas.present();
        }
    }

    Ok(())
}

fn load_media(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<HashMap<KeyPress, Texture>, String> {
    let mut textures_map = HashMap::new();
    textures_map.insert(
        KeyPress::Up,
        texture_creator.load_texture("resources/up.bmp")?,
    );
    textures_map.insert(
        KeyPress::Down,
        texture_creator.load_texture("resources/down.bmp")?,
    );
    textures_map.insert(
        KeyPress::Left,
        texture_creator.load_texture("resources/left.bmp")?,
    );
    textures_map.insert(
        KeyPress::Right,
        texture_creator.load_texture("resources/right.bmp")?,
    );
    textures_map.insert(
        KeyPress::Press,
        texture_creator.load_texture("resources/press.bmp")?,
    );

    Ok(textures_map)
}
