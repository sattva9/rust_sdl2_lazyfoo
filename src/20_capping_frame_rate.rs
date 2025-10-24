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
const SCREEN_FPS: f32 = 60.0;
const SCREEN_TICKS_PER_FRAME: f32 = 1000.0 / SCREEN_FPS;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let timer_subsystem = sdl_context.timer()?;
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

    let font = ttf_context.load_font("resources/lazy.ttf", 32)?;
    let text_color = Color::RGB(0, 0, 0);

    let time_text_prompt_texture = LTexture::load_from_rendered_text(
        &texture_creator,
        &font,
        "Average Frames Per Second (With Cap):",
        text_color,
    )?;

    //The frames per second timer
    let mut fps_timer = LTimer::new(sdl_context.timer()?);

    //The frames per second cap timer
    let mut cap_timer = LTimer::new(sdl_context.timer()?);

    //Start counting frames per second
    let mut counted_frames = 0;
    fps_timer.start();

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        cap_timer.start();

        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'app;
            }
        }

        //Calculate and correct fps
        let mut avg_fps = counted_frames as f32 / (fps_timer.get_ticks() as f32 / 1000.0);
        if avg_fps > 2000000.0 {
            avg_fps = 0.0;
        }

        //Set text to be rendered
        let time_text = format!("{avg_fps:.2}");

        let time_texture =
            LTexture::load_from_rendered_text(&texture_creator, &font, &time_text, text_color)?;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

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
            (time_text_texture_y + time_texture.height) as i32,
            None,
        )?;
        canvas.present();

        counted_frames += 1;

        //If frame finished early
        let frame_ticks = cap_timer.get_ticks();
        if frame_ticks < SCREEN_TICKS_PER_FRAME as u32 {
            //Wait remaining time
            timer_subsystem.delay(SCREEN_TICKS_PER_FRAME as u32 - frame_ticks);
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
}
