use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const TOTAL_BUTTONS: u32 = 4;
const BUTTON_WIDTH: u32 = 300;
const BUTTON_HEIGHT: u32 = 200;

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

    let (button_texture, clip_rects) = load_media(&texture_creator)?;
    let mut buttons = initialize_buttons();

    let mut event_pump = sdl_context.event_pump()?;
    'app: loop {
        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'app;
            }
        }

        let state = event_pump.mouse_state();
        for i in 0..TOTAL_BUTTONS {
            buttons[i as usize].handle_event(&state);
        }

        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        for i in 0..TOTAL_BUTTONS {
            buttons[i as usize].render(&mut canvas, &button_texture, &clip_rects)?;
        }

        canvas.present();
    }

    Ok(())
}

#[derive(Copy, Clone)]
enum LButtonSprite {
    ButtonSpriteMouseOut = 0,
    ButtonSpriteMouseOverMotion,
    ButtonSpriteMouseDown,
    ButtonSpriteMouseUp,
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

struct LButton {
    position: Point,
    current_sprite: LButtonSprite,
    pressed: bool,
}

impl LButton {
    fn new() -> LButton {
        LButton {
            position: Point::new(0, 0),
            current_sprite: LButtonSprite::ButtonSpriteMouseOut,
            pressed: false,
        }
    }

    fn new_from_point(p: Point) -> LButton {
        LButton {
            position: p,
            current_sprite: LButtonSprite::ButtonSpriteMouseOut,
            pressed: false,
        }
    }

    fn handle_event(&mut self, mouse_state: &MouseState) {
        if (mouse_state.x() < self.position.x())
            || (mouse_state.x() > self.position.x() + BUTTON_WIDTH as i32)
            || (mouse_state.y() < self.position.y())
            || (mouse_state.y() > self.position.y() + BUTTON_HEIGHT as i32)
        {
            self.current_sprite = LButtonSprite::ButtonSpriteMouseOut;
        } else {
            self.current_sprite = match mouse_state.left() {
                true => {
                    self.pressed = true;
                    LButtonSprite::ButtonSpriteMouseDown
                }
                false => {
                    if self.pressed == true {
                        LButtonSprite::ButtonSpriteMouseUp
                    } else {
                        LButtonSprite::ButtonSpriteMouseOverMotion
                    }
                }
            }
        }
    }

    fn render(
        &self,
        canvas: &mut WindowCanvas,
        texture: &LTexture,
        clips: &Vec<Rect>,
    ) -> Result<(), String> {
        let indx = self.current_sprite as usize;
        texture.render(
            canvas,
            self.position.x(),
            self.position.y(),
            Some(clips[indx]),
            None,
            None,
            false,
            false,
        )
    }
}

fn load_media(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<(LTexture, Vec<Rect>), String> {
    let button_sprite = LTexture::load_from_file(texture_creator, "resources/button.png")?;

    let mut clip_rects: Vec<Rect> = Vec::new();
    for i in 0..TOTAL_BUTTONS {
        clip_rects.push(Rect::new(0, i as i32 * 200, BUTTON_WIDTH, BUTTON_HEIGHT));
    }
    Ok((button_sprite, clip_rects))
}

fn initialize_buttons() -> [LButton; 4] {
    [
        LButton::new(),
        LButton::new_from_point(Point::new((SCREEN_WIDTH - BUTTON_WIDTH) as i32, 0)),
        LButton::new_from_point(Point::new(0, (SCREEN_HEIGHT - BUTTON_HEIGHT) as i32)),
        LButton::new_from_point(Point::new(
            (SCREEN_WIDTH - BUTTON_WIDTH) as i32,
            (SCREEN_HEIGHT - BUTTON_HEIGHT) as i32,
        )),
    ]
}
