use rand::Rng;

use super::*;
use std::{iter, sync, thread, time, vec};

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

    pub fn new_galaxy_black_hole(
        num_particles: u32,
        radius: f64,
        mass: f64,
        color: (u8, u8, u8),
    ) -> Self {
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
                position,
                velocity: Vector2 { x: 0.0, y: 0.0 },
                color,
            });
        }

        world.add_particle(Particle {
            mass: mass / 2.0,
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            color,
        });

        world.set_circle_speed();

        world
    }

    pub fn new_galaxy(num_particles: u32, radius: f64, mass: f64, color: (u8, u8, u8)) -> Self {
        let mut world = Self::new();
        world.gravity_strength = 0.1;

        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let distance = (rng.gen::<f64>()).powi(2) * radius;
            let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let position = Vector2 {
                x: distance * angle.cos(),
                y: distance * angle.sin(),
            };
            world.add_particle(Particle {
                mass: mass / num_particles as f64 / 2.0,
                position,
                velocity: Vector2 { x: 0.0, y: 0.0 },
                color,
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

    fn calculate_gravity_quadtree(&self, quadtree: &Quadtree, position: Vector2) -> Vector2 {
        let mut gravity = Vector2 { x: 0.0, y: 0.0 };

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

    pub fn update_quadtree(&mut self, delta_time: f64) {
        let start_time = time::Instant::now();

        let mut min = self.particles[0].position;
        let mut max = self.particles[0].position;
        for particle in &self.particles {
            min.x = min.x.min(particle.position.x);
            min.y = min.y.min(particle.position.y);
            max.x = max.x.max(particle.position.x);
            max.y = max.y.max(particle.position.y);
        }

        let mut quadtree = Quadtree::new(min, max);
        for particle in &self.particles {
            quadtree.insert(particle.position, particle.mass, 0);
        }
        //println!("{:?}", quadtree);

        let elapsed_time = start_time.elapsed();
        println!("Quadtree initialization: {}ms", elapsed_time.as_millis());

        let mut forces = vec![];
        for particle in &self.particles {
            let gravity = self.calculate_gravity_quadtree(&quadtree, particle.position);
            let force = gravity * particle.mass;
            forces.push(force)
        }

        for (particle, force) in iter::zip(&mut self.particles, forces) {
            particle.update(delta_time, force);
        }
    }

    pub fn add_position(&mut self, position: Vector2) {
        for particle in &mut self.particles {
            particle.position += position;
        }
    }

    pub fn add_velocity(&mut self, velocity: Vector2) {
        for particle in &mut self.particles {
            particle.velocity += velocity;
        }
    }

    pub fn add_world(&mut self, other: &Self) {
        for particle in &other.particles {
            self.add_particle(particle.clone());
        }
    }
}

#[derive(Debug)]
struct QuadtreeNode {
    min: Vector2,
    max: Vector2,
    children: Option<[usize; 4]>,

    position: Vector2,
    mass: f64,
}

impl QuadtreeNode {
    fn new(min: Vector2, max: Vector2) -> Self {
        Self {
            min,
            max,
            children: None,
            position: Vector2 { x: 0.0, y: 0.0 },
            mass: 0.0,
        }
    }
    fn which_child(&self, vector: Vector2) -> usize {
        let mut i = 0;
        if vector.x > (self.min.x + self.max.x) / 2.0 {
            i += 1;
        }
        if vector.y > (self.min.y + self.max.y) / 2.0 {
            i += 2;
        }
        i
    }
}

#[derive(Debug)]
struct Quadtree {
    nodes: Vec<QuadtreeNode>,
}

impl Quadtree {
    fn new(min: Vector2, max: Vector2) -> Self {
        Self {
            nodes: vec![QuadtreeNode::new(min, max)],
        }
    }

    fn insert(&mut self, position: Vector2, mass: f64, node: usize) {
        if let Some(children) = self.nodes[node].children {
            let child = children[self.nodes[node].which_child(position)];
            self.insert(position, mass, child);

            let mut new_position = Vector2 { x: 0.0, y: 0.0 };
            let mut new_mass = 0.0;
            for i in 0..4 {
                let current_child = &self.nodes[children[i]];
                new_position += current_child.position * current_child.mass;
                new_mass += current_child.mass;
            }

            self.nodes[node].position = new_position / new_mass;
            self.nodes[node].mass = new_mass;
        } else {
            if self.nodes[node].mass > 0.0 {
                let temp_position = self.nodes[node].position;
                let temp_mass = self.nodes[node].mass;

                self.add_children(node);

                self.insert(temp_position, temp_mass, node);
                self.insert(position, mass, node)
            } else {
                self.nodes[node].position = position;
                self.nodes[node].mass = mass;
            }
        }
    }

    fn add_children(&mut self, node: usize) {
        let mut children = [0usize; 4];
        for i in 0..4 {
            children[i] = self.nodes.len();
            let mut min = self.nodes[node].min;
            let mut max = self.nodes[node].max;
            if i % 2 == 0 {
                max.x = (min.x + max.x) / 2.0;
            } else {
                min.x = (min.x + max.x) / 2.0;
            }
            if i / 2 == 0 {
                max.y = (min.y + max.y) / 2.0;
            } else {
                min.y = (min.y + max.y) / 2.0;
            }
            self.nodes.push(QuadtreeNode::new(min, max));
        }
        self.nodes[node].children = Some(children);
    }
}

pub fn test_quadtree() {
    let mut quad_tree = Quadtree::new(Vector2 { x: 0.0, y: 0.0 }, Vector2 { x: 1.0, y: 1.0 });

    quad_tree.insert(Vector2 { x: 0.2, y: 0.2 }, 1.0, 0);
    quad_tree.insert(Vector2 { x: 0.8, y: 0.8 }, 1.0, 0);
    quad_tree.insert(Vector2 { x: 0.2, y: 0.8 }, 1.0, 0);
    quad_tree.insert(Vector2 { x: 0.8, y: 0.2 }, 1.0, 0);
    println!("{:?}", quad_tree);
}
