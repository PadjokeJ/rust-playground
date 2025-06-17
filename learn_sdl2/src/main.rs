use sdl2::keyboard::Keycode;
use sdl2::event::Event;

extern crate sdl2;

fn main() -> Result<(), String>{
    let res: (u32, u32) = (400, 300);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Hello World", res.0, res.1)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                _ => (),
            }
        }
    }

    Ok(())
}
