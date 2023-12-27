use super::*;
use rand::Rng;
use std::f64::consts::PI;

fn milky_way() -> World {
    todo!();
}

pub fn from_distrobution(
    density_fn: fn(radius: f64, z: f64) -> f64,
    total_mass: f64,
    num_particles: u32,
    r_max: f64,
    z_max: f64,
    steps_r: u32,
    steps_z: u32,
) -> World {
    let mut world = World::new(WorldSettings {
        gravity_strength: 0.0,
        softening_length: 0.0,
        accuracy: 0.0,
        quadtree: false,
        multiprocessing: false,
    });

    let mass_per_particle = total_mass / num_particles as f64;

    let r_delta = r_max / steps_r as f64;
    let z_delta = z_max / steps_z as f64;
    let mut rng = rand::thread_rng();
    for r_index in 0..steps_r {
        let r = r_index as f64 * r_delta;
        let mut mass = 0.0;
        for z_index in 0..steps_z {
            let z = z_index as f64 * z_delta;

            let density = density_fn(r, z);
            let volume = PI * ((r + r_delta).powi(2) - r * r) * z_delta;
            mass += density * volume;
        }

        let n = ((mass / total_mass) * num_particles as f64) as u32;
        for i in 0..n {
            let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let position = Vector2 {
                x: angle.cos() * r,
                y: angle.sin() * r,
            };
            world.add_particle(Particle {
                mass: mass_per_particle,
                position,
                velocity: Vector2 { x: 0.0, y: 0.0 },
                color: (1.0, 1.0, 1.0),
            });
        }
    }

    world
}
