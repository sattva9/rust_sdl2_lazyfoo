use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
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

    let prompt_text_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        "Enter Text:",
        Color::RGB(0, 0, 0),
    )?;

    let mut input_text = "Some Text".to_string();
    let mut input_text_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        &input_text,
        Color::RGB(0, 0, 0),
    )?;

    video_subsystem.text_input().start();

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        let mut rendered_text = false;

        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'app,
                Event::KeyDown {
                    keycode, keymod, ..
                } => {
                    if keycode.eq(&Some(Keycode::BACKSPACE)) && input_text.len() > 0 {
                        input_text.pop();
                        rendered_text = true;
                    } else if keycode.eq(&Some(Keycode::C)) && keymod.eq(&Mod::LCTRLMOD) {
                        let _ = video_subsystem.clipboard().set_clipboard_text(&input_text);
                    } else if keycode.eq(&Some(Keycode::V)) && keymod.eq(&Mod::LCTRLMOD) {
                        if let Ok(tmp_text) = video_subsystem.clipboard().clipboard_text() {
                            input_text = tmp_text;
                            rendered_text = true;
                        }
                    }
                }
                Event::TextInput { text, .. } => {
                    input_text.push_str(&text);
                    rendered_text = true;
                }
                _ => {}
            }
        }

        if rendered_text {
            if input_text.is_empty() {
                input_text = " ".to_string();
            }

            input_text_texture = LTexture::load_from_rendered_text(
                &texture_creator,
                &font,
                &input_text,
                Color::RGB(0, 0, 0),
            )?;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let _ = prompt_text_texture.render(
            &mut canvas,
            (SCREEN_WIDTH as i32 - prompt_text_texture.width as i32) / 2,
            0,
            None,
        );
        let _ = input_text_texture.render(
            &mut canvas,
            (SCREEN_WIDTH as i32 - input_text_texture.width as i32) / 2,
            input_text_texture.height as i32,
            None,
        );

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

#[allow(dead_code)]
fn load_media<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> Result<LTexture<'a>, String> {
    LTexture::load_from_file(texture_creator, path)
}
