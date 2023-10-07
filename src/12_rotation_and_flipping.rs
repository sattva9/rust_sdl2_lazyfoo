use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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

    let arrow = LTexture::load_from_file(&texture_creator, "resources/arrow.png")?;

    let mut degrees: f64 = 0.0;
    let mut flip_vertical: bool = false;
    let mut flip_horizontal: bool = false;

    'app: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::A) => {
                        degrees -= 60.0;
                    }
                    Some(Keycode::D) => {
                        degrees += 60.0;
                    }
                    Some(Keycode::Q) => {
                        flip_horizontal = !flip_horizontal;
                    }
                    Some(Keycode::W) => {
                        flip_horizontal = false;
                        flip_vertical = false;
                    }
                    Some(Keycode::E) => {
                        flip_vertical = !flip_vertical;
                    }
                    Some(Keycode::Escape) => {
                        break 'app;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        arrow.render(
            &mut canvas,
            (SCREEN_WIDTH - arrow.width) as i32 / 2,
            (SCREEN_HEIGHT - arrow.height) as i32 / 2,
            None,
            Some(degrees),
            None,
            flip_horizontal,
            flip_vertical,
        )?;

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
        let mut surface = Surface::from_file(path)?;
        surface.set_color_key(true, Color::RGB(0, 255, 255))?;
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
        rotation: Option<f64>,
        center: Option<Point>,
        flip_h: bool,
        flip_v: bool,
    ) -> Result<(), String> {
        let rect = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };
        let rotation: f64 = match rotation {
            Some(rot) => rot,
            None => 0.0,
        };
        canvas.copy_ex(&self.texture, clip, rect, rotation, center, flip_h, flip_v)?;
        Ok(())
    }
}
