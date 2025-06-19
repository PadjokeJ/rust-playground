mod vectors;

struct RigidBody2D {
    pos: V2,
    vel: V2,
    gravity: f32,
    gravity_direction: V2,
    resistance: f32,
}

trait Gravity {
    fn apply_gravity(&mut self, delta_time: f32);
}

trait Move {
    fn move(&mut self, delta_time: f32);
    fn apply_resistance(&mut self, delta_time: f32);
}

impl RigidBody2D {
    fn new(pos: V2) -> RigidBody2D {
        RigidBody2D {pos: pos, vel: V2::new(), gravity: 9.81, gravity_direction: V2::new(0.0, 1.0), resistance: 0.1}
    }
}

impl Move for RigidBody2D {
    fn move(&mut self, delta_time: f32) {
        self.pos += vel * delta_time;
    }
    fn apply_resistance(&mut self, delta_time: f32) {
        self.vel -= self.vel * self.resistance * delta_time;
    }
}

impl Gravity for RigidBody2D {
    fn apply_gravity(&mut self, delta_time: f32) {
        self.vel += self.gravity_direction * self.gravity * delta_time;
    }
}