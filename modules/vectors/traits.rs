mod v2;

trait Norm {
    fn norm(self) -> f32;
    fn sqr_norm(self) -> f32;
    fn normalized(self) -> Self;
    fn normalize(&mut self);
}

trait Distance {
    fn distance_to(self, other: V2) -> f32;
    fn sqr_distance_to(self, other: V2) -> f32;
}