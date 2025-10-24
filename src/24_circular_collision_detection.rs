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

struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

impl Circle {
    fn new(x: i32, y: i32, r: i32) -> Self {
        Self { x, y, r }
    }
}

struct Dot {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
    collider: Circle,
}

impl Dot {
    const DOT_WIDTH: u32 = 20;
    const DOT_HEIGHT: u32 = 20;
    const DOT_VEL: i32 = 1;

    fn new(x: i32, y: i32) -> Self {
        let collider = Circle::new(x, y, Self::DOT_WIDTH as i32 / 2);
        Self {
            x_pos: x,
            y_pos: y,
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

    fn move_position(&mut self, square: &Rect, circle: &Circle) {
        //Move the dot left or right
        self.x_pos += self.x_vel;
        self.shift_colliders();

        //If the dot went too far to the left or right
        if self.x_pos - self.collider.r < 0
            || (self.x_pos + self.collider.r as i32 > SCREEN_WIDTH as i32)
            || check_collision1(&self.collider, square)
            || check_collision2(&self.collider, circle)
        {
            //Move back
            self.x_pos -= self.x_vel;
            self.shift_colliders();
        }

        //Move the dot up or down
        self.y_pos += self.y_vel;
        self.shift_colliders();

        //If the dot went too far up or down
        if self.y_pos - self.collider.r < 0
            || (self.y_pos + self.collider.r as i32 > SCREEN_HEIGHT as i32)
            || check_collision1(&self.collider, square)
            || check_collision2(&self.collider, circle)
        {
            //Move back
            self.y_pos -= self.y_vel;
            self.shift_colliders();
        }
    }

    fn render(&self, dot_texture: &LTexture, canvas: &mut WindowCanvas) {
        let _ = dot_texture.render(
            canvas,
            self.x_pos - self.collider.r,
            self.y_pos - self.collider.r,
            None,
        );
    }

    fn get_collider(&self) -> &Circle {
        &self.collider
    }

    fn shift_colliders(&mut self) {
        self.collider.x = self.x_pos;
        self.collider.y = self.y_pos;
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

    let mut dot = Dot::new(Dot::DOT_WIDTH as i32 / 2, Dot::DOT_HEIGHT as i32 / 2);
    let other_dot = Dot::new(SCREEN_WIDTH as i32 / 4, SCREEN_HEIGHT as i32 / 4);
    let wall = Rect::new(300, 40, 40, 400);

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'app,
                _ => dot.handle_event(event),
            }
        }

        dot.move_position(&wall, other_dot.get_collider());

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        //Render wall
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.draw_rect(wall)?;

        other_dot.render(&dot_texture, &mut canvas);
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

fn check_collision1(a: &Circle, b: &Rect) -> bool {
    //Closest point on collision box
    //Find closest x offset
    let closest_x = if a.x < b.x {
        b.x
    } else if a.x > b.x + b.w {
        b.x + b.w
    } else {
        a.x
    };

    //Find closest y offset
    let closest_y = if a.y < b.y {
        b.y
    } else if a.y > b.y + b.h {
        b.y + b.w
    } else {
        a.y
    };

    //If the closest point is inside the circle
    if distance_squared(a.x, a.y, closest_x, closest_y) < a.r * a.r {
        //This box and the circle have collided
        return true;
    }

    false
}

fn check_collision2(a: &Circle, b: &Circle) -> bool {
    //Calculate total radius squared
    let mut total_radius_squared = a.r + b.r;
    total_radius_squared = total_radius_squared * total_radius_squared;

    //If the distance between the centers of the circles is less than the sum of their radii
    if distance_squared(a.x, a.y, b.x, b.y) < total_radius_squared {
        //The circles have collided
        return true;
    }

    false
}

fn distance_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let delta_x = x2 - x1;
    let delta_y = y2 - y1;

    delta_x * delta_x + delta_y * delta_y
}
