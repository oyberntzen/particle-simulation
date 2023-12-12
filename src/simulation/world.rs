use std::{iter, fs, io, io::Write};
use super::*;

pub struct World {
    particles: Vec<Particle>,
    pub gravity_strength: f64,
    pub softening_length: f64,
}

impl World {
    pub fn new() -> Self {
        Self {
            particles: vec![],
            gravity_strength: 0.1,
            softening_length: 0.1,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn calculate_gravity(&self, position: Vector2) -> Vector2 {
        let mut gravity = Vector2{x: 0.0, y: 0.0};
        for particle in &self.particles {
            if particle.position.x == position.x && particle.position.y == position.y {
                continue;
            }
            let difference = particle.position - position;
            let distance = difference.abs();
            let direction = difference / distance;
            let magnitude = self.gravity_strength * particle.mass / (distance*distance + self.softening_length*self.softening_length);
            gravity += direction * magnitude;
        }
        gravity
    }

    pub fn update(&mut self, delta_time: f64) {
        let mut forces = vec![];
        for particle in &self.particles {
            let gravity = self.calculate_gravity(particle.position);
            let force = gravity * particle.mass;
            forces.push(force)
        }

        for (particle, force)  in iter::zip(&mut self.particles, forces) {
            particle.update(delta_time, force);
        }
    }

    pub fn write_to_file(&self, file: &mut fs::File) -> io::Result<()> {
        for particle in &self.particles {
            writeln!(file, "{} {}", particle.position.x, particle.position.y)?;
        }
        Ok(())
    }
}
