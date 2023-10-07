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

    let mut texture = LTexture::load_from_file(&texture_creator, "resources/colors.png")?;

    let mut red_tint: u8 = 255;
    let mut green_tint: u8 = 255;
    let mut blue_tint: u8 = 255;

    'app: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) => {
                        break 'app;
                    }
                    Some(Keycode::Q) => {
                        if red_tint < 224 {
                            red_tint += 32;
                        }
                    }
                    Some(Keycode::W) => {
                        if green_tint < 224 {
                            green_tint += 32;
                        }
                    }
                    Some(Keycode::E) => {
                        if blue_tint < 224 {
                            blue_tint += 32;
                        }
                    }
                    Some(Keycode::A) => {
                        if red_tint > 32 {
                            red_tint -= 32;
                        }
                    }
                    Some(Keycode::S) => {
                        if green_tint > 32 {
                            green_tint -= 32;
                        }
                    }
                    Some(Keycode::D) => {
                        if blue_tint > 32 {
                            blue_tint -= 32;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        texture.set_color(red_tint, green_tint, blue_tint);
        texture.render(&mut canvas, 0, 0, None)?;

        canvas.present();
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
        let surface = Surface::from_file(path)?;
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| format!("error while creating texture. {e}"))?;
        Ok(Self::new(texture))
    }

    fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.texture.set_color_mod(r, g, b);
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
        canvas.copy(&self.texture, rect, rect)?;
        Ok(())
    }
}
