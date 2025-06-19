use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::{keyboard::Keycode, render::Canvas};

use std::cmp::min;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use rand::Rng;

extern crate sdl2;

#[derive(Clone, Copy)]
struct Asteroid {
    pos: V2,
    vel: V2,
    radius: i32,
    speed: f32,
}
impl Move for Asteroid {
    fn update_vel(&mut self, add: V2, delta: f32) {
        self.vel += add * delta;
    }
    fn update_pos(&mut self, delta: f32) {
        self.pos += self.vel * delta * self.speed;
    }
}
struct Player {
    pos: V2,
    vel: V2,
    speed: f32,
    radius: f32,
}
impl Move for Player {
    fn update_vel(&mut self, add: V2, delta: f32) {
        self.vel += add * delta;
    }
    fn update_pos(&mut self, delta: f32) {
        self.pos += self.vel * delta * self.speed;
    }
}
trait Move {
    fn update_vel(&mut self, add: V2, delta: f32);
    fn update_pos(&mut self, delta: f32);
}

trait Norm {
    fn norm(self) -> f32;
    fn sqr_norm(self) -> f32;
    fn normalized(self) -> Self;
    fn normalize(&mut self);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct V2 {
    x: f32,
    y: f32,
}
impl Add for V2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
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
        Self {
            x: self.x * other,
            y: self.y * other,
        }
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
    fn sqr_norm(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    fn normalized(self) -> Self {
        if self.norm() > 0.0 {
            Self {
                x: self.x / self.norm(),
                y: self.y / self.norm(),
            }
        } else {
            self
        }
    }
    fn normalize(&mut self) {
        let vec = self.normalized();
        self.x = vec.x;
        self.y = vec.y;
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

fn main() -> Result<(), String> {
    let mut rng = rand::rng();

    let res: (u32, u32) = (640, 480);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Asteroids", res.0, res.1)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut input_vec: (bool, bool, bool, bool) = (false, false, false, false);

    let mut delta_time: f32 = 0f32;

    let mut player: Player = Player {
        pos: V2 {
            x: (res.0 as i32 / 2) as f32,
            y: (res.1 as i32 / 2) as f32,
        },
        vel: V2 { x: 0.0, y: 0.0 },
        speed: 24f32,
        radius: 5.0,
    };

    let mut shooting: bool = false;
    let mut bullets: Vec<V2> = Vec::new();
    let mut bullets_rot: Vec<V2> = Vec::new();
    let bullet_delay = Duration::from_secs_f32(0.1);
    let mut time_since_last_bullet = Duration::from_secs(0);

    let mut asteroids: Vec<Asteroid> = Vec::new();

    let spawn_delay = Duration::from_secs_f32(0.5);
    let mut time_since_last_spawn = Duration::from_secs(0);

    'main: loop {
        let start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => input_vec.0 = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => input_vec.1 = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => input_vec.2 = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => input_vec.3 = true,
                Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => shooting = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => input_vec.0 = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => input_vec.1 = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => input_vec.2 = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => input_vec.3 = false,
                Event::KeyUp {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => shooting = false,

                _ => (),
            }
        }

        if time_since_last_spawn > spawn_delay {
            time_since_last_spawn -= spawn_delay;

            let pos: V2 = V2 {
                x: (rng.random::<i32>() % 2) as f32 * (res.0 as i32 as f32),
                y: (rng.random::<i32>() % 2) as f32 * (res.0 as i32 as f32),
            };
            let vel: V2 = V2 {
                x: (rng.random::<i32>() % 11 - 5) as f32,
                y: (rng.random::<i32>() % 11 - 5) as f32,
            }
            .normalized();

            let mut radius: i32 = rng.random::<u32>() as i32;
            radius = radius % 20 + 20;
            let mut speed: i32 = rng.random::<u32>() as i32;
            speed = speed % 30 + 60;

            asteroids.push(Asteroid {
                pos: pos,
                vel: vel,
                radius: radius,
                speed: speed as f32,
            });
        }
        time_since_last_spawn += Duration::from_secs_f32(delta_time);

        for ast in &mut asteroids {
            ast.pos += ast.vel * ast.speed * delta_time;

            let pos_x = ast.pos.x;
            let pos_y = ast.pos.y;
            let radius = ast.radius as f32;

            if pos_x < -radius {
                ast.pos.x = radius + res.0 as i32 as f32;
            }
            if pos_x > res.0 as i32 as f32 + radius {
                ast.pos.x = -radius;
            }
            if pos_y < -radius {
                ast.pos.y = radius + res.1 as i32 as f32;
            }
            if pos_y > res.1 as i32 as f32 + radius {
                ast.pos.y = -radius;
            }

            if (player.pos - ast.pos).norm() < player.radius + ast.radius as f32 {
                loop {
                    for event in event_pump.poll_iter() {
                        match event {
                            Event::Quit { .. }
                            | Event::KeyDown {
                                keycode: Some(Keycode::Escape),
                                ..
                            } => break 'main,
                            _ => (),
                        }
                    }
                }
            }
        }

        let mov_vec = V2 {
            x: (input_vec.3 as i32 - input_vec.1 as i32) as f32,
            y: (input_vec.2 as i32 - input_vec.0 as i32) as f32,
        }
        .normalized();

        player.update_vel(mov_vec * player.speed - player.vel * 0.1, delta_time);
        player.update_pos(delta_time);

        time_since_last_bullet += Duration::from_secs_f32(delta_time);
        time_since_last_bullet = min(time_since_last_bullet, bullet_delay);
        if shooting && time_since_last_bullet >= bullet_delay && mov_vec.sqr_norm() > 0.0 {
            time_since_last_bullet -= bullet_delay;

            bullets.push(V2 {
                x: player.pos.x,
                y: player.pos.y,
            });
            bullets_rot.push(mov_vec);
        }

        let mut i = 0;
        for _ in 0..bullets.len() {
            bullets[i].x += bullets_rot[i].x * delta_time * 750f32;
            bullets[i].y += bullets_rot[i].y * delta_time * 750f32;

            let a = asteroids.clone();
            let mut j = 0;
            let mut removed = false;
            for ast in a {
                if (ast.pos - bullets[i]).norm() < ast.radius as f32 + 5.0 {
                    asteroids.remove(j);
                    bullets.remove(i);
                    bullets_rot.remove(i);
                    removed = true;
                    break;
                } else {
                    j += 1;
                }
            }

            if !removed {
                if bullets.len() > 0
                    && (bullets[i].x - (res.0 / 2) as i32 as f32).abs() > (res.0 / 2) as i32 as f32
                    || (bullets[i].y - (res.1 / 2) as i32 as f32).abs() > (res.1 / 2) as i32 as f32
                {
                    bullets.remove(i);
                    bullets_rot.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        if player.pos.x < -player.radius {
            player.pos.x = player.radius + res.0 as i32 as f32;
        }
        if player.pos.x > res.0 as i32 as f32 + player.radius {
            player.pos.x = -player.radius;
        }
        if player.pos.y < -player.radius {
            player.pos.y = player.radius + res.1 as i32 as f32;
        }
        if player.pos.y > res.1 as i32 as f32 + player.radius {
            player.pos.y = -player.radius;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        draw_circle(
            &mut canvas,
            player.pos.x as i32,
            player.pos.y as i32,
            player.radius as i32,
        );

        canvas.set_draw_color(Color::RGB(150, 0, 50));
        for ast in &asteroids {
            draw_circle(&mut canvas, ast.pos.x as i32, ast.pos.y as i32, ast.radius);
        }

        canvas.set_draw_color(Color::RGB(0, 150, 50));
        for i in 0..bullets.len() {
            let b = bullets[i];
            for j in 0..10 {
                let br = bullets_rot[i] * 2.0 * j as f32 + b;

                draw_circle(&mut canvas, br.x as i32, br.y as i32, 2);
            }
        }

        canvas.present();

        delta_time = start.elapsed().as_secs_f32();
    }

    Ok(())
}
