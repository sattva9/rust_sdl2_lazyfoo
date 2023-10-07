use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
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

    let font = ttf_context.load_font("resources/gnd.ttf", 32)?;

    let prompt_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        "Press Enter to Reset Start Time.",
        Color::RGB(255, 225, 0),
    )?;
    let time_text_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        "Seconds since start time: ",
        Color::RGB(255, 255, 255),
    )?;
    let mut start_time = 0;
    let timer = sdl_context.timer()?;

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
                    Keycode::Return => {
                        start_time = timer.ticks();
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let time_texture = LTexture::load_from_rendered_text(
            &texture_creator,
            &font,
            &format!("{}", (timer.ticks() - start_time) / 1000),
            Color::RGB(255, 255, 255),
        )?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        prompt_texture.render(
            &mut canvas,
            ((SCREEN_WIDTH - prompt_texture.width) / 2) as i32,
            100,
            None,
        )?;
        let time_text_texture_y = (SCREEN_HEIGHT - time_text_texture.height) / 2;
        time_text_texture.render(
            &mut canvas,
            ((SCREEN_WIDTH - time_text_texture.width) / 2) as i32,
            (time_text_texture_y) as i32,
            None,
        )?;
        time_texture.render(
            &mut canvas,
            ((SCREEN_WIDTH - time_texture.width) / 2) as i32,
            (time_text_texture_y + time_text_texture.height + 10) as i32,
            None,
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
            .blended(color)
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
    ) -> Result<(), String> {
        let rect = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };
        canvas.copy(&self.texture, clip, rect)
    }
}
