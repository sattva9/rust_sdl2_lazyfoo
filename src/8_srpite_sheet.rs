use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

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

    let (sprite_texture, sprite_clips) = load_media(&texture_creator, "resources/dots.png")?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    sprite_texture.render(&mut canvas, 0, 0, Some(sprite_clips[0]))?;
    sprite_texture.render(
        &mut canvas,
        (SCREEN_WIDTH - sprite_clips[1].width()) as i32,
        0,
        Some(sprite_clips[1]),
    )?;
    sprite_texture.render(
        &mut canvas,
        0,
        (SCREEN_HEIGHT - sprite_clips[2].height()) as i32,
        Some(sprite_clips[2]),
    )?;
    sprite_texture.render(
        &mut canvas,
        (SCREEN_WIDTH - sprite_clips[3].width()) as i32,
        (SCREEN_HEIGHT - sprite_clips[3].height()) as i32,
        Some(sprite_clips[3]),
    )?;
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

struct LTexture<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'a> LTexture<'a> {
    fn new(texture: Texture<'a>) -> Self {
        let width = texture.query().width;
        let height = texture.query().height;
        Self {
            texture,
            width,
            height,
        }
    }

    fn load_from_file(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &str,
    ) -> Result<Self, String> {
        let mut surface = Surface::from_file(path)?;
        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff))?;
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| format!("error while creating texture. {e}"))?;
        Ok(Self::new(texture))
    }

    fn render(
        &self,
        canvas: &mut WindowCanvas,
        x: i32,
        y: i32,
        clip: Option<Rect>,
    ) -> Result<(), String> {
        let rect = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };
        canvas.copy(&self.texture, clip, rect)?;
        Ok(())
    }
}

fn load_media<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> Result<(LTexture<'a>, [Rect; 4]), String> {
    Ok((
        LTexture::load_from_file(texture_creator, path)?,
        [
            Rect::new(0, 0, 100, 100),
            Rect::new(100, 0, 100, 100),
            Rect::new(0, 100, 100, 100),
            Rect::new(100, 100, 100, 100),
        ],
    ))
}
