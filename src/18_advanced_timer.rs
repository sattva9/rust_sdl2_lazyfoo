use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::TimerSubsystem;

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

    let start_prompt_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        "Press S to start or stop the timer",
        Color::RGB(255, 225, 0),
    )?;
    let pause_prompt_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        "Press P to pause or unpause the timer",
        Color::RGB(255, 255, 0),
    )?;
    let time_text_prompt_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        "Seconds since start time:",
        Color::RGB(255, 255, 255),
    )?;

    let mut l_timer = LTimer::new(sdl_context.timer()?);

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
                    Keycode::S => {
                        if l_timer.is_started() {
                            l_timer.stop();
                        } else {
                            l_timer.start()
                        }
                        break;
                    }
                    Keycode::P => {
                        if l_timer.is_paused() {
                            l_timer.unpause();
                        } else {
                            l_timer.pause()
                        }
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
            &format!("{}", l_timer.get_ticks() / 1000),
            Color::RGB(255, 255, 255),
        )?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        start_prompt_texture.render(
            &mut canvas,
            ((SCREEN_WIDTH - start_prompt_texture.width) / 2) as i32,
            0,
            None,
        )?;
        pause_prompt_texture.render(
            &mut canvas,
            ((SCREEN_WIDTH - pause_prompt_texture.width) / 2) as i32,
            start_prompt_texture.height as i32,
            None,
        )?;
        let time_text_texture_y = (SCREEN_HEIGHT - time_text_prompt_texture.height) / 2;
        time_text_prompt_texture.render(
            &mut canvas,
            ((SCREEN_WIDTH - time_text_prompt_texture.width) / 2) as i32,
            (time_text_texture_y) as i32,
            None,
        )?;
        time_texture.render(
            &mut canvas,
            ((SCREEN_WIDTH - time_texture.width) / 2) as i32,
            (time_text_texture_y + time_text_prompt_texture.height) as i32,
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

struct LTimer {
    start_ticks: u32,
    paused_ticks: u32,
    paused: bool,
    started: bool,
    timer: TimerSubsystem,
}

impl LTimer {
    fn new(timer: TimerSubsystem) -> Self {
        Self {
            start_ticks: 0,
            paused_ticks: 0,
            paused: false,
            started: false,
            timer,
        }
    }

    fn start(&mut self) {
        self.started = true;
        self.paused = false;
        self.start_ticks = self.timer.ticks();
        self.paused_ticks = 0;
    }

    fn stop(&mut self) {
        self.started = false;
        self.paused = false;
        self.start_ticks = 0;
        self.paused_ticks = 0;
    }

    fn pause(&mut self) {
        if self.started && !self.paused {
            self.paused = true;
            self.paused_ticks = self.timer.ticks() - self.start_ticks;
            self.start_ticks = 0;
        }
    }

    fn unpause(&mut self) {
        if self.started && self.paused {
            self.paused = false;
            self.start_ticks = self.timer.ticks() - self.paused_ticks;
            self.paused_ticks = 0;
        }
    }

    fn get_ticks(&self) -> u32 {
        if self.started {
            return if self.paused {
                self.paused_ticks
            } else {
                self.timer.ticks() - self.start_ticks
            };
        }
        0
    }

    fn is_started(&self) -> bool {
        self.started
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}
