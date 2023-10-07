use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{Channel, Chunk, Music, DEFAULT_FORMAT};
use std::path::Path;

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
    let texture = texture_creator.load_texture("resources/prompt.png")?;

    canvas.clear();
    canvas.copy(&texture, None, None)?;
    canvas.present();

    sdl2::mixer::open_audio(44100, DEFAULT_FORMAT, 2, 2048)?;
    let music_media = load_media()?;
    let channel = Channel::all();

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => {
                        break 'app;
                    }
                    Keycode::Num1 => {
                        channel.play(&music_media.high, 0)?;
                        break;
                    }
                    Keycode::Num2 => {
                        channel.play(&music_media.medium, 0)?;
                        break;
                    }
                    Keycode::Num3 => {
                        channel.play(&music_media.low, 0)?;
                        break;
                    }
                    Keycode::Num4 => {
                        channel.play(&music_media.scratch, 0)?;
                        break;
                    }
                    Keycode::Num9 => {
                        if !Music::is_playing() {
                            music_media.music.play(0)?;
                        } else {
                            if Music::is_paused() {
                                Music::resume();
                            } else {
                                Music::pause();
                            }
                        }
                    }
                    Keycode::Num0 => {
                        Music::halt();
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(())
}

struct MusicMedia<'a> {
    music: Music<'a>,
    scratch: Chunk,
    high: Chunk,
    medium: Chunk,
    low: Chunk,
}

fn load_media<'a>() -> Result<MusicMedia<'a>, String> {
    Ok(MusicMedia {
        music: Music::from_file(Path::new("resources/beat.wav"))?,
        scratch: Chunk::from_file(Path::new("resources/scratch.wav"))?,
        high: Chunk::from_file(Path::new("resources/high.wav"))?,
        medium: Chunk::from_file(Path::new("resources/medium.wav"))?,
        low: Chunk::from_file(Path::new("resources/low.wav"))?,
    })
}
