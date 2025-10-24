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
    colliders: Vec<Rect>,
}

impl Dot {
    const DOT_WIDTH: u32 = 20;
    const DOT_HEIGHT: u32 = 20;
    const DOT_VEL: i32 = 1;

    fn new(x: i32, y: i32) -> Self {
        let mut colliders: Vec<Rect> = vec![Rect::new(0, 0, 0, 0); 11];

        //Initialize the collision boxes' width and height
        colliders[0].w = 6;
        colliders[0].h = 1;

        colliders[1].w = 10;
        colliders[1].h = 1;

        colliders[2].w = 14;
        colliders[2].h = 1;

        colliders[3].w = 16;
        colliders[3].h = 2;

        colliders[4].w = 18;
        colliders[4].h = 2;

        colliders[5].w = 20;
        colliders[5].h = 6;

        colliders[6].w = 18;
        colliders[6].h = 2;

        colliders[7].w = 16;
        colliders[7].h = 2;

        colliders[8].w = 14;
        colliders[8].h = 1;

        colliders[9].w = 10;
        colliders[9].h = 1;

        colliders[10].w = 6;
        colliders[10].h = 1;

        let mut dot = Self {
            x_pos: x,
            y_pos: y,
            x_vel: 0,
            y_vel: 0,
            colliders,
        };

        dot.shift_colliders();

        dot
    }

    fn shift_colliders(&mut self) {
        //The row offset
        let mut r = 0;

        //Go through the dot's collision boxes
        for set in 0..self.colliders.len() {
            //Center the collision box
            self.colliders[set].x =
                self.x_pos + (Self::DOT_WIDTH as i32 - self.colliders[set].w) / 2;

            //Set the collision box at its row offset
            self.colliders[set].y = self.y_pos + r;

            //Move the row offset down the height of the collision box
            r += self.colliders[set].h;
        }
    }

    fn get_colliders(&self) -> &Vec<Rect> {
        &self.colliders
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

    fn move_position(&mut self, other_colliders: &Vec<Rect>) {
        //Move the dot left or right
        self.x_pos += self.x_vel;
        self.shift_colliders();

        //If the dot went too far to the left or right
        if self.x_pos < 0
            || (self.x_pos + Self::DOT_WIDTH as i32 > SCREEN_WIDTH as i32)
            || check_collision(&self.colliders, other_colliders)
        {
            //Move back
            self.x_pos -= self.x_vel;
            self.shift_colliders();
        }

        //Move the dot up or down
        self.y_pos += self.y_vel;
        self.shift_colliders();

        //If the dot went too far up or down
        if self.y_pos < 0
            || (self.y_pos + Self::DOT_HEIGHT as i32 > SCREEN_HEIGHT as i32)
            || check_collision(&self.colliders, other_colliders)
        {
            //Move back
            self.y_pos -= self.y_vel;
            self.shift_colliders();
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

    let mut dot = Dot::new(0, 0);
    let other_dot = Dot::new((SCREEN_WIDTH / 4) as i32, (SCREEN_HEIGHT / 4) as i32);

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'app,
                _ => dot.handle_event(event),
            }
        }

        dot.move_position(&other_dot.get_colliders());

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        dot.render(&dot_texture, &mut canvas);
        other_dot.render(&dot_texture, &mut canvas);

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

fn check_collision(a: &Vec<Rect>, b: &Vec<Rect>) -> bool {
    for a_box in 0..a.len() {
        let left_a = a[a_box].x;
        let right_a = a[a_box].x + a[a_box].w;
        let top_a = a[a_box].y;
        let bottom_a = a[a_box].y + a[a_box].h;

        for b_box in 0..b.len() {
            let left_b = b[b_box].x;
            let right_b = b[b_box].x + b[b_box].w;
            let top_b = b[b_box].y;
            let bottom_b = b[b_box].y + b[b_box].h;

            //If no sides from A are outside of B
            if ((bottom_a <= top_b)
                || (top_a >= bottom_b)
                || (right_a <= left_b)
                || (left_a >= right_b))
                == false
            {
                //A collision is detected
                return true;
            }
        }
    }

    false
}
