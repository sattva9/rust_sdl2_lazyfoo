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

struct Dot {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
    collider: Rect,
}

impl Dot {
    const DOT_WIDTH: u32 = 20;
    const DOT_HEIGHT: u32 = 20;
    const DOT_VEL: i32 = 1;

    fn new() -> Self {
        let collider = Rect::new(0, 0, Self::DOT_WIDTH, Self::DOT_HEIGHT);
        Self {
            x_pos: 0,
            y_pos: 0,
            x_vel: 0,
            y_vel: 0,
            collider,
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

    fn move_position(&mut self, wall: &Rect) {
        //Move the dot left or right
        self.x_pos += self.x_vel;
        self.collider.x = self.x_pos;

        //If the dot went too far to the left or right
        if self.x_pos < 0
            || (self.x_pos + Self::DOT_WIDTH as i32 > SCREEN_WIDTH as i32)
            || check_collision(&self.collider, wall)
        {
            //Move back
            self.x_pos -= self.x_vel;
            self.collider.x = self.x_pos;
        }

        //Move the dot up or down
        self.y_pos += self.y_vel;
        self.collider.y = self.y_pos;

        //If the dot went too far up or down
        if self.y_pos < 0
            || (self.y_pos + Self::DOT_HEIGHT as i32 > SCREEN_HEIGHT as i32)
            || check_collision(&self.collider, wall)
        {
            //Move back
            self.y_pos -= self.y_vel;
            self.collider.y = self.y_pos;
        }
    }

    fn render(&self, dot_texture: &LTexture, canvas: &mut WindowCanvas) {
        let _ = dot_texture.render(canvas, self.x_pos, self.y_pos, None);
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

    let mut dot = Dot::new();
    let wall = Rect::new(300, 40, 40, 400);

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'app,
                _ => dot.handle_event(event),
            }
        }

        dot.move_position(&wall);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        //Render wall
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.draw_rect(wall)?;

        dot.render(&dot_texture, &mut canvas);

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

fn check_collision(a: &Rect, b: &Rect) -> bool {
    let left_a = a.x;
    let right_a = a.x + a.w;
    let top_a = a.y;
    let bottom_a = a.y + a.h;

    let left_b = b.x;
    let right_b = b.x + b.w;
    let top_b = b.y;
    let bottom_b = b.y + b.h;

    //If any of the sides from A are outside of B
    if bottom_a <= top_b {
        return false;
    }

    if top_a >= bottom_b {
        return false;
    }

    if right_a <= left_b {
        return false;
    }

    if left_a >= right_b {
        return false;
    }

    //If none of the sides from A are outside B
    true
}
