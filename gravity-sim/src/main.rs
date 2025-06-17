use std::time::Instant;

use sdl2::video::Window;
use sdl2::{keyboard::Keycode, render::Canvas};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::ops::{Add, Sub, Mul, MulAssign, AddAssign, SubAssign};

extern crate sdl2;

struct Planet {
    pos: V2,
    mass: i32,
    vel: V2,
}


trait Norm {
    fn norm(self) -> f32;
}
trait SquareNorm {
    fn sqr_norm(self) -> f32;
}
trait Normalize {
    fn normalize(self) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct V2 {
    x: f32,
    y: f32,
}
impl Add for V2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}
impl AddAssign for V2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Sub for V2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}
impl SubAssign for V2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl Mul for V2 {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}
impl Mul<f32> for V2 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self { x: self.x * other, y: self.y * other }
    }
}
impl MulAssign<f32> for V2 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}
impl Norm for V2 {
    fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
impl SquareNorm for V2 {
    fn sqr_norm(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}
impl Normalize for V2 {
    fn normalize(self) -> Self {
        if self.norm() > 0.0{
            Self {x: self.x / self.norm(), y: self.y / self.norm()}
        }
        else{
            self
        }
    }
}

fn draw_circle(canvas: &mut Canvas<Window>, mid_x: i32, mid_y: i32, radius: i32) {
    let mut x = radius - 1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (2 * radius);

    while x >= y {
        let _ = canvas.draw_point(Point::new(mid_x + x, mid_y + y));
        let _ = canvas.draw_point(Point::new(mid_x + y, mid_y + x));
        let _ = canvas.draw_point(Point::new(mid_x - y, mid_y + x));
        let _ = canvas.draw_point(Point::new(mid_x - x, mid_y + y));
        let _ = canvas.draw_point(Point::new(mid_x - x, mid_y - y));
        let _ = canvas.draw_point(Point::new(mid_x - y, mid_y - x));
        let _ = canvas.draw_point(Point::new(mid_x + y, mid_y - x));
        let _ = canvas.draw_point(Point::new(mid_x + x, mid_y - y));
        
        if err <= 0 {
            y += 1;
            err += dy;
            dy += 2;
        }

        if err > 0 {
            x -= 1;
            dx += 2;
            err += dx - (2 * radius);
        }
    }
}

fn main() -> Result<(), String>{
    let res: (u32, u32) = (640, 480);

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

    let mut pos: V2 = V2 {x: 0.0, y: 0.0};
    let mut vel: V2 = V2 {x: 0.0, y: 0.0};
    let mass = 10;

    let speed = 80f32;

    let mut planet = Planet {pos: V2 { x: 300f32, y: 200f32}, mass: 10, vel: V2 { x: 0f32, y: 0f32}};

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

        let mov_vec = V2{x: (input_vec.3 as i32 - input_vec.1 as i32) as f32, y: (input_vec.2 as i32 - input_vec.0 as i32) as f32};
        
        
        

        let force = (mass * planet.mass) as f32 / (pos - planet.pos).sqr_norm();

        let a = force / planet.mass as f32;
        planet.vel +=(pos - planet.pos).normalize() * a;
        planet.pos += planet.vel * delta_time;

        vel += (planet.pos - pos).normalize() * (force / mass as f32);
        pos += vel * delta_time + mov_vec * delta_time * speed;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(150, 0, 50));
        let _ = draw_circle(&mut canvas, pos.x as i32, pos.y as i32, 30);
        let _ = draw_circle(&mut canvas, planet.pos.x as i32, planet.pos.y as i32, 20);
        canvas.present();

        delta_time = start.elapsed().as_secs_f32();
    }

    Ok(())
}
