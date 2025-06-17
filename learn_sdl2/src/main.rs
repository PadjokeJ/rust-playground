use std::time::Instant;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

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

    let mut input_vec: (bool, bool, bool, bool) = (false, false, false, false);

    let mut pos: (f32, f32) = (200f32, 150f32);
    let mut vel: (f32, f32) = (0f32, 0f32);

    let speed = 1f32;

    let mut delta_time: f32 = 0f32;

    'main: loop {
        let start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                Event::KeyDown {keycode: Some(Keycode::W), ..} => input_vec.0 = true,
                Event::KeyDown {keycode: Some(Keycode::A), ..} => input_vec.1 = true,
                Event::KeyDown {keycode: Some(Keycode::S), ..} => input_vec.2 = true,
                Event::KeyDown {keycode: Some(Keycode::D), ..} => input_vec.3 = true,
                Event::KeyUp {keycode: Some(Keycode::W), ..} => input_vec.0 = false,
                Event::KeyUp {keycode: Some(Keycode::A), ..} => input_vec.1 = false,
                Event::KeyUp {keycode: Some(Keycode::S), ..} => input_vec.2 = false,
                Event::KeyUp {keycode: Some(Keycode::D), ..} => input_vec.3 = false,
                _ => (),
            }
        }

        let mov_vec = (input_vec.3 as i32 - input_vec.1 as i32, input_vec.2 as i32 - input_vec.0 as i32);
        vel = (vel.0 + mov_vec.0 as f32, vel.1 + mov_vec.1 as f32);
        vel = (vel.0 - (vel.0 * 0.2 * delta_time * 60f32), vel.1 - (vel.1 * 0.2 * delta_time * 60f32));
        pos = (pos.0 + vel.0 * delta_time * speed, pos.1 + vel.1 * delta_time * speed);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(150, 0, 50));
        let _ = canvas.draw_rect(Rect::new(pos.0 as i32, pos.1 as i32, 50, 50));
        canvas.present();

        delta_time = start.elapsed().as_secs_f32();
    }

    Ok(())
}
