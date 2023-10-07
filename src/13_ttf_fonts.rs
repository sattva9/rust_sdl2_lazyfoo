use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::{Font, Sdl2TtfContext};
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

    let ttf_context =
        sdl2::ttf::init().map_err(|e| format!("Could not initialize sdl2_ttf. {e}"))?;
    let text = load_media(&texture_creator, &ttf_context)?;

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

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        text.render(
            &mut canvas,
            (SCREEN_WIDTH - text.width) as i32 / 2,
            (SCREEN_HEIGHT - text.height) as i32 / 2,
            None,
            None,
            None,
            false,
            false,
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

    fn load_from_rendered_text(
        texture_creator: &'a TextureCreator<WindowContext>,
        font: &Font,
        text: &str,
        color: Color,
    ) -> Result<Self, String> {
        let text_surface = font
            .render(text)
            // .blended(color)
            .solid(color)
            .map_err(|e| format!("Could not create text surface. {e}"))?;

        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface)
            .map_err(|e| format!("Could not convert text surface to texture. {e}"))?;

        Ok(LTexture::new(text_texture))
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

fn load_media<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    ttf: &'a Sdl2TtfContext,
) -> Result<LTexture<'a>, String> {
    let font = ttf.load_font("resources/gnd.ttf", 26)?;

    LTexture::load_from_rendered_text(
        texture_creator,
        &font,
        "The quick brown fox jumps over the lazy dog",
        Color::RGB(255, 0, 0),
    )
}
