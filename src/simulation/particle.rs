use super::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Particle {
    pub mass: f64,
    pub position: Vector2,
    pub velocity: Vector2,
    pub color: (f64, f64, f64),
}

impl Particle {
    pub fn update(&mut self, delta_time: f64, force: Vector2) {
        let acceleration = force / self.mass;
        self.velocity += acceleration * delta_time;
        self.position += self.velocity * delta_time;
    }
}
