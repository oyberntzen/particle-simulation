use rand::Rng;

use super::*;
use std::{iter, sync, thread, vec};

#[derive(Clone)]
pub struct World {
    pub particles: Vec<Particle>,
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

    pub fn new_galaxy_black_hole(num_particles: u32, radius: f64, mass: f64) -> Self {
        let mut world = Self::new();
        world.gravity_strength = 0.1;

        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let distance = rng.gen::<f64>() * radius + radius * 0.02;
            let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let position = Vector2 {
                x: distance * angle.cos(),
                y: distance * angle.sin(),
            };
            world.add_particle(Particle {
                mass: mass / num_particles as f64 / 2.0,
                position: position,
                velocity: Vector2 { x: 0.0, y: 0.0 },
            });
        }

        world.add_particle(Particle {
            mass: mass / 2.0,
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
        });

        world.set_circle_speed();

        world
    }

    pub fn new_galaxy(num_particles: u32, radius: f64, mass: f64) -> Self {
        let mut world = Self::new();
        world.gravity_strength = 0.1;

        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let distance = (rng.gen::<f64>() * radius).powi(2);
            let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let position = Vector2 {
                x: distance * angle.cos(),
                y: distance * angle.sin(),
            };
            world.add_particle(Particle {
                mass: mass / num_particles as f64 / 2.0,
                position: position,
                velocity: Vector2 { x: 0.0, y: 0.0 },
            });
        }

        world.set_circle_speed();

        world
    }

    fn set_circle_speed(&mut self) {
        let mut start_velocities: Vec<Vector2> = vec![];
        for particle in &self.particles {
            if particle.position.x == 0.0 && particle.position.y == 0.0 {
                continue;
            }
            let acceleration = self.calculate_gravity(particle.position);
            let velocity = (acceleration.abs() * particle.position.abs()).sqrt();

            let vector_to_center = (-particle.position) / particle.position.abs();
            let velocity_vector = Vector2 {
                x: vector_to_center.y,
                y: -vector_to_center.x,
            } * velocity;
            //println!("{} {} {} {}", velocity, particle.position, vector_to_center, velocity_vector);

            start_velocities.push(velocity_vector);
        }

        for (particle, velocity) in iter::zip(&mut self.particles, start_velocities) {
            particle.velocity = velocity;
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn calculate_gravity(&self, position: Vector2) -> Vector2 {
        let mut gravity = Vector2 { x: 0.0, y: 0.0 };
        for particle in &self.particles {
            if particle.position.x == position.x && particle.position.y == position.y {
                continue;
            }
            let difference = particle.position - position;
            let distance = difference.abs();
            let direction = difference / distance;
            let magnitude = self.gravity_strength * particle.mass
                / (distance * distance + self.softening_length * self.softening_length);
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

        for (particle, force) in iter::zip(&mut self.particles, forces) {
            particle.update(delta_time, force);
        }
    }

    pub fn update_multiprocessing(&mut self, delta_time: f64) {
        let num_threads = thread::available_parallelism().unwrap().get();
        let mut handles = vec![];
        let particles_per_thread = (self.particles.len() + num_threads - 1) / num_threads;
        let world = sync::Arc::new(self.clone());
        for i in 0..num_threads {
            let current_world = world.clone();
            handles.push(thread::spawn(move || {
                let mut forces = vec![];
                for j in i * particles_per_thread..(i + 1) * particles_per_thread {
                    if j >= current_world.particles.len() {
                        break;
                    }
                    let particle = current_world.particles[j].clone();
                    let gravity = current_world.calculate_gravity(particle.position);
                    let force = gravity * particle.mass;
                    forces.push(force);
                }
                forces
            }));
        }

        let mut all_forces = vec![];
        for handle in handles {
            let forces = handle.join().unwrap();
            for force in forces {
                all_forces.push(force);
            }
        }

        for (particle, force) in iter::zip(&mut self.particles, all_forces) {
            particle.update(delta_time, force);
        }
    }
}
