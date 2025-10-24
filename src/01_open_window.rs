use sdl2::event::Event;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 280;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let _window = video_subsystem
        .window("first window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| format!("error while initializing window. {e}"))?;

    'app: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'app;
            }
        }
    }
    Ok(())
}
