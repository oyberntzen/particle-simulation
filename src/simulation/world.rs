use rand::Rng;

use super::*;
use std::{iter, sync, thread, time, vec};
use std::io::Write; 
use std::fs;

#[derive(Clone)]
pub struct World {
    pub particles: Vec<Particle>,
    pub settings: WorldSettings,
}

impl World {
    pub fn new(settings: WorldSettings) -> Self {
        Self {
            particles: vec![],
            settings,
        }
    }

    pub fn new_galaxy_black_hole(
        &mut self,
        num_particles: u32,
        radius: f64,
        mass: f64,
        color: (f64, f64, f64),
    ) {
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let distance = rng.gen::<f64>() * radius + radius * 0.02;
            let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let position = Vector2 {
                x: distance * angle.cos(),
                y: distance * angle.sin(),
            };
            self.add_particle(Particle {
                mass: mass / num_particles as f64 / 2.0,
                position,
                velocity: Vector2 { x: 0.0, y: 0.0 },
                color,
            });
        }

        self.add_particle(Particle {
            mass: mass / 2.0,
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            color,
        });

        self.set_circle_speed();
    }

    pub fn new_galaxy(
        &mut self,
        num_particles: u32,
        radius: f64,
        mass: f64,
        color: (f64, f64, f64),
    ) {
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let distance = (rng.gen::<f64>()).powi(2) * radius;
            let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let position = Vector2 {
                x: distance * angle.cos(),
                y: distance * angle.sin(),
            };
            self.add_particle(Particle {
                mass: mass / num_particles as f64 / 2.0,
                position,
                velocity: Vector2 { x: 0.0, y: 0.0 },
                color,
            });
        }

        self.set_circle_speed();
    }

    pub fn set_circle_speed(&mut self) {
        let forces = self.calculate_forces_auto();
        let mut start_velocities: Vec<Vector2> = vec![];
        for (particle, force) in iter::zip(&self.particles, forces) {
            if particle.position.x == 0.0 && particle.position.y == 0.0 {
                //println!("KULT");
                start_velocities.push(Vector2 { x: 0.0, y: 0.0 });
                continue;
            }
            let acceleration = force / particle.mass;
            let velocity = (acceleration.abs() * particle.position.abs()).sqrt();

            let vector_to_center = (-particle.position) / particle.position.abs();
            let velocity_vector = Vector2 {
                x: vector_to_center.y,
                y: -vector_to_center.x,
            } * velocity;

            start_velocities.push(velocity_vector);
        }

        for (particle, velocity) in iter::zip(&mut self.particles, start_velocities) {
            particle.velocity = velocity;
            //println!("{}", particle.velocity)
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
            let magnitude = self.settings.gravity_strength * particle.mass
                / (distance * distance
                    + self.settings.softening_length * self.settings.softening_length);
            gravity += direction * magnitude;
        }
        gravity
    }

    fn calculate_forces_auto(&self) -> Vec<Vector2> {
        let start_time = time::Instant::now();
        let forces = match (self.settings.multiprocessing, self.settings.quadtree) {
            (false, false) => self.calculate_forces(),
            (true, false) => self.calculate_forces_multiprocessing(),
            (false, true) => self.calculate_forces_quadtree(),
            (true, true) => self.calculate_forces_multiprocessing_quadtree(),
        };
        let elapsed_time = start_time.elapsed();
        println!("Total time: {}ms", elapsed_time.as_millis());
        forces
    }

    fn calculate_forces(&self) -> Vec<Vector2> {
        let mut forces = vec![];
        for particle in &self.particles {
            let gravity = self.calculate_gravity(particle.position);
            let force = gravity * particle.mass;
            forces.push(force);
        }
        forces
    }

    fn calculate_forces_multiprocessing(&self) -> Vec<Vector2> {
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
        all_forces
    }

    fn calculate_forces_quadtree(&self) -> Vec<Vector2> {
        let start_time = time::Instant::now();

        let quadtree = self.construct_quadtree();

        let elapsed_time = start_time.elapsed();
        println!("Quadtree initialization: {}ms", elapsed_time.as_millis());

        let start_time = time::Instant::now();

        let mut forces = vec![];
        for particle in &self.particles {
            let gravity = quadtree.calculate_gravity(particle.position, 0, &self.settings);
            let force = gravity * particle.mass;
            forces.push(force);
        }

        let elapsed_time = start_time.elapsed();
        println!("Force calculation: {}ms", elapsed_time.as_millis());
        forces
    }

    fn calculate_forces_multiprocessing_quadtree(&self) -> Vec<Vector2> {
        let start_time = time::Instant::now();

        let num_threads = thread::available_parallelism().unwrap().get();
        let mut handles = vec![];
        let particles_per_thread = (self.particles.len() + num_threads - 1) / num_threads;
        let world = sync::Arc::new(self.clone());
        let quadtree = sync::Arc::new(self.construct_quadtree());

        let elapsed_time = start_time.elapsed();
        println!("Quadtree initialization: {}ms", elapsed_time.as_millis());

        let start_time = time::Instant::now();
        for i in 0..num_threads {
            let current_world = world.clone();
            let current_quadtree = quadtree.clone();
            handles.push(thread::spawn(move || {
                let mut forces = vec![];
                for j in i * particles_per_thread..(i + 1) * particles_per_thread {
                    if j >= current_world.particles.len() {
                        break;
                    }
                    let particle = current_world.particles[j].clone();
                    let gravity = current_quadtree.calculate_gravity(
                        particle.position,
                        0,
                        &current_world.settings,
                    );
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

        let elapsed_time = start_time.elapsed();
        println!("Force calculation: {}ms", elapsed_time.as_millis());
        all_forces
    }

    fn construct_quadtree(&self) -> Quadtree {
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
        quadtree
    }

    pub fn update(&mut self, delta_time: f64) {
        let forces = self.calculate_forces_auto();

        let mut i = 0;
        for (particle, force) in iter::zip(&mut self.particles, forces) {
            //println!("{}: {}", i, particle.position);
            particle.update(delta_time, force);
            //println!("{}: {}", i, particle.position);
            i += 1;
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

    pub fn set_color(&mut self, color: (f64, f64, f64)) {
        for particle in &mut self.particles {
            particle.color = color;
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let encoded = bincode::serialize(&self.particles).unwrap();
        let mut file = fs::OpenOptions::new().write(true).open(path).unwrap();
        let _ = file.write_all(&encoded);
    }
}

#[derive(Clone)]
pub struct WorldSettings {
    pub gravity_strength: f64,
    pub softening_length: f64,
    pub accuracy: f64,
    pub quadtree: bool,
    pub multiprocessing: bool,
}

#[derive(Debug)]
struct QuadtreeNode {
    min: Vector2,
    max: Vector2,
    children: Option<[usize; 4]>,
    depth: usize,

    position: Vector2,
    mass: f64,
}

impl QuadtreeNode {
    fn new(min: Vector2, max: Vector2, depth: usize) -> Self {
        Self {
            min,
            max,
            children: None,
            position: Vector2 { x: 0.0, y: 0.0 },
            mass: 0.0,
            depth,
        }
    }
    fn which_child(&self, position: Vector2) -> usize {
        let mut i = 0;
        if position.x > (self.min.x + self.max.x) / 2.0 {
            i += 1;
        }
        if position.y > (self.min.y + self.max.y) / 2.0 {
            i += 2;
        }
        i
    }
    fn inside(&self, position: Vector2) -> bool {
        if position.x < self.min.x {
            return false;
        }
        if position.x > self.max.x {
            return false;
        }
        if position.y < self.min.y {
            return false;
        }
        if position.y > self.max.y {
            return false;
        }
        true
    }
}

#[derive(Debug)]
struct Quadtree {
    nodes: Vec<QuadtreeNode>,
}

impl Quadtree {
    fn new(min: Vector2, max: Vector2) -> Self {
        Self {
            nodes: vec![QuadtreeNode::new(min, max, 0)],
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
                if self.nodes[node].depth == 32 {
                    self.nodes[node].mass += mass;
                    return;
                }

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
            self.nodes
                .push(QuadtreeNode::new(min, max, self.nodes[node].depth + 1));
        }
        self.nodes[node].children = Some(children);
    }

    fn calculate_gravity(
        &self,
        position: Vector2,
        node: usize,
        settings: &WorldSettings,
    ) -> Vector2 {
        let current_node = &self.nodes[node];
        let distance = (position - current_node.position).abs().sqrt();
        let width = current_node.max.x - current_node.min.x;
        let height = current_node.max.y - current_node.min.y;
        let size = width.max(height);

        let far_away = size / distance < settings.accuracy;
        let has_children = current_node.children.is_some();
        let inside = current_node.inside(position);

        if inside && !has_children {
            Vector2 { x: 0.0, y: 0.0 }
        } else if (inside || !far_away) && has_children {
            // search children
            let mut gravity = Vector2 { x: 0.0, y: 0.0 };
            for i in 0..4 {
                gravity +=
                    self.calculate_gravity(position, current_node.children.unwrap()[i], settings);
            }
            gravity
        } else {
            let difference = current_node.position - position;
            let distance = difference.abs();
            let direction = difference / distance;
            let magnitude = settings.gravity_strength * current_node.mass
                / (distance * distance + settings.softening_length * settings.softening_length);
            let gravity = direction * magnitude;
            gravity
        }
    }
}
