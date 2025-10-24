use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

const LEVEL_WIDTH: u32 = 1280;
const LEVEL_HEIGHT: u32 = 960;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

struct Dot {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Dot {
    const DOT_WIDTH: u32 = 20;
    const DOT_HEIGHT: u32 = 20;
    const DOT_VEL: i32 = 1;

    fn new(x: i32, y: i32) -> Self {
        Self {
            x_pos: x,
            y_pos: y,
            x_vel: 0,
            y_vel: 0,
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            //If a key was pressed
            Event::KeyDown {
                keycode, repeat, ..
            } => {
                if !repeat {
                    match keycode {
                        Some(Keycode::UP) => self.y_vel -= Self::DOT_VEL,
                        Some(Keycode::DOWN) => self.y_vel += Self::DOT_VEL,
                        Some(Keycode::LEFT) => self.x_vel -= Self::DOT_VEL,
                        Some(Keycode::RIGHT) => self.x_vel += Self::DOT_VEL,
                        _ => {}
                    }
                }
            }
            //If a key was released
            Event::KeyUp {
                keycode, repeat, ..
            } => {
                if !repeat {
                    match keycode {
                        Some(Keycode::UP) => self.y_vel += Self::DOT_VEL,
                        Some(Keycode::DOWN) => self.y_vel -= Self::DOT_VEL,
                        Some(Keycode::LEFT) => self.x_vel += Self::DOT_VEL,
                        Some(Keycode::RIGHT) => self.x_vel -= Self::DOT_VEL,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn move_position(&mut self) {
        //Move the dot left or right
        self.x_pos += self.x_vel;

        //If the dot went too far to the left or right
        if self.x_pos < 0 || (self.x_pos + Self::DOT_WIDTH as i32 > LEVEL_WIDTH as i32) {
            //Move back
            self.x_pos -= self.x_vel;
        }

        //Move the dot up or down
        self.y_pos += self.y_vel;

        //If the dot went too far up or down
        if self.y_pos < 0 || (self.y_pos + Self::DOT_HEIGHT as i32 > LEVEL_HEIGHT as i32) {
            //Move back
            self.y_pos -= self.y_vel;
        }
    }

    //Shows the dot on the screen relative to the camera
    fn render(&self, dot_texture: &LTexture, canvas: &mut WindowCanvas, cam_x: i32, cam_y: i32) {
        let _ = dot_texture.render(canvas, self.x_pos - cam_x, self.y_pos - cam_y, None);
    }
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
    let dot_texture = load_media(&texture_creator, "resources/dot.bmp")?;
    let bg_texture = load_media(&texture_creator, "resources/bg.png")?;

    let mut dot = Dot::new(Dot::DOT_WIDTH as i32 / 2, Dot::DOT_HEIGHT as i32 / 2);
    let mut camera = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'app,
                _ => dot.handle_event(event),
            }
        }

        dot.move_position();

        //Center the camera over the dot
        camera.x = (dot.x_pos + Dot::DOT_WIDTH as i32 / 2) - SCREEN_WIDTH as i32 / 2;
        camera.y = (dot.y_pos + Dot::DOT_HEIGHT as i32 / 2) - SCREEN_HEIGHT as i32 / 2;

        //Keep the camera in bounds
        if camera.x < 0 {
            camera.x = 0;
        }
        if camera.y < 0 {
            camera.y = 0;
        }
        if camera.x > LEVEL_WIDTH as i32 - camera.w {
            camera.x = LEVEL_WIDTH as i32 - camera.w;
        }
        if camera.y > LEVEL_HEIGHT as i32 - camera.h {
            camera.y = LEVEL_HEIGHT as i32 - camera.h;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        //Render background
        let _ = bg_texture.render(&mut canvas, 0, 0, Some(camera));

        dot.render(&dot_texture, &mut canvas, camera.x, camera.y);

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
    ) -> Result<(), String> {
        let rect = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };
        canvas.copy(&self.texture, clip, rect)
    }
}

fn load_media<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> Result<LTexture<'a>, String> {
    LTexture::load_from_file(texture_creator, path)
}
