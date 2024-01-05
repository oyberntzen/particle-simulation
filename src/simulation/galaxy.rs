use super::*;
use rand::Rng;
use std::{f64::consts::PI, net::ToSocketAddrs};

fn milky_way() -> World {
    todo!();
}

pub fn distrobution_mass(density_fn: fn(radius: f64, z: f64) -> f64, r_max: f64, z_max: f64, steps_r: u32, steps_z: u32) -> f64 {
    let r_delta = r_max / steps_r as f64;
    let z_delta = z_max / steps_z as f64 * 2.0;
    let mut rng = rand::thread_rng();
    let mut total_mass = 0.0;
    for r_index in 0..steps_r {
        let r = r_index as f64 * r_delta;
        for z_index in 0..steps_z {
            let z = z_index as f64 * z_delta - z_max;

            let density = density_fn(r, z);
            let volume = PI * ((r + r_delta).powi(2) - r * r) * z_delta;
            total_mass += density * volume;
        }
    }
    total_mass
}

pub fn from_distrobution(
    density_fn: fn(radius: f64, z: f64) -> f64,
    num_particles: u32,
    r_max: f64,
    z_max: f64,
    steps_r: u32,
    steps_z: u32,
) -> World {
    let total_mass = distrobution_mass(density_fn, r_max, z_max, steps_r, steps_z);

    let mut world = World::new(WorldSettings {
        gravity_strength: 0.0,
        softening_length: 0.0,
        accuracy: 0.0,
        quadtree: false,
        multiprocessing: false,
    });

    let mass_per_particle = total_mass / num_particles as f64;
    println!("{} {}",total_mass, mass_per_particle);

    let r_delta = r_max / steps_r as f64;
    let z_delta = z_max / steps_z as f64 * 2.0;
    let mut rng = rand::thread_rng();
    for r_index in 0..steps_r {
        let r = r_index as f64 * r_delta;
        let mut mass = 0.0;
        for z_index in 0..steps_z {
            let z = z_index as f64 * z_delta - z_max;

            let density = density_fn(r, z);
            let volume = PI * ((r + r_delta).powi(2) - r * r) * z_delta;
            mass += density * volume;
        }

        let n = ((mass / total_mass) * num_particles as f64) as u32;
        for _ in 0..n {
            let angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let distance = r + r_delta * rng.gen::<f64>();
            let position = Vector2 {
                x: angle.cos() * distance,
                y: angle.sin() * distance,
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

pub fn milkyway() -> World {
    let bulge_density = |r: f64, z: f64| {
        let a = 1.8;
        let r0 = 0.075;
        let rcut = 2.1;
        let q = 0.5;
        let p0b = 98.4e9;

        let rp = (r * r + (z / q).powi(2)).sqrt();
        p0b / (1.0 + rp / r0).powf(a) * (-(rp / rcut).powi(2)).exp()
    };

    let thin_disc_density = |r: f64, z: f64| {
        let sigma0 = 886.7e6;
        let zd = 2.6;
        let rd = 2.53;

        sigma0/(2.0*zd)*(-z.abs()/zd-r/rd).exp()
    };

    let thick_disc_density = |r: f64, z: f64| {
        let sigma0 = 156.7e6;
        let zd = 3.6;
        let rd = 3.38;

        sigma0/(2.0*zd)*(-z.abs()/zd-r/rd).exp()
    };

    let gas_disc1_density = |r: f64, z: f64| {
        let sigma0 = 53.1e6;
        let zd = 0.085;
        let rm = 4.0;
        let rd = 7.0;

        let x = z / (2.0*zd);
        sigma0/(4.0*zd)*(-rm/r-r/rd).exp()*(2.0/(x.exp()+(-x).exp())).powi(2)
    };

    let gas_disc2_density = |r: f64, z: f64| {
        let sigma0 = 2180.0e6;
        let zd = 0.045;
        let rm = 12.0;
        let rd = 1.5;

        let x = z / (2.0*zd);
        sigma0/(4.0*zd)*(-rm/r-r/rd).exp()*(2.0/(x.exp()+(-x).exp())).powi(2)
    };

    let r_max = 25.0;
    let z_max = 0.5;
    let steps_r = 1000;
    let steps_z = 100;

    let bulge_mass = distrobution_mass(bulge_density, r_max, z_max, steps_r, steps_z);
    let thin_disc_mass = distrobution_mass(thin_disc_density, r_max, z_max, steps_r, steps_z);
    let thick_disc_mass = distrobution_mass(thick_disc_density, r_max, z_max, steps_r, steps_z);
    let gas_disc1_mass = distrobution_mass(gas_disc1_density, r_max, z_max, steps_r, steps_z);
    let gas_disc2_mass = distrobution_mass(gas_disc2_density, r_max, z_max, steps_r, steps_z);
    
    let total_mass = bulge_mass + thin_disc_mass + thick_disc_mass + gas_disc1_mass + gas_disc2_mass;

    let total_particles = 100000;
    let bulge_particles = (bulge_mass / total_mass * total_particles as f64) as u32;
    let thin_disc_particles = (thin_disc_mass / total_mass * total_particles as f64) as u32;
    let thick_disc_particles = (thick_disc_mass / total_mass * total_particles as f64) as u32;
    let gas_disc1_particles = (gas_disc1_mass / total_mass * total_particles as f64) as u32;
    let gas_disc2_particles = (gas_disc2_mass / total_mass * total_particles as f64) as u32;

    let mut bulge_world = from_distrobution(bulge_density, bulge_particles, r_max, z_max, steps_r, steps_z);

    let mut thin_disc_world = from_distrobution(thin_disc_density, thin_disc_particles, r_max, z_max, steps_r, steps_z);

    let mut thick_disc_world = from_distrobution(thick_disc_density, thick_disc_particles, r_max, z_max, steps_r, steps_z);

    let mut gas_disc1_world = from_distrobution(gas_disc1_density, gas_disc1_particles, r_max, z_max, steps_r, steps_z);

    let mut gas_disc2_world = from_distrobution(gas_disc2_density, gas_disc2_particles, r_max, z_max, steps_r, steps_z);

    let settings = WorldSettings {
        gravity_strength: 1.30128e-12, //1e20 aar
        softening_length: 0.1,
        accuracy: 0.5,
        quadtree: true,
        multiprocessing: true,
    };

    bulge_world.set_color((1.0, 0.0, 0.0));
    thin_disc_world.set_color((0.0, 1.0, 0.0));
    thick_disc_world.set_color((0.0, 1.0, 0.0));
    gas_disc1_world.set_color((0.0, 0.0, 1.0));
    gas_disc2_world.set_color((0.0, 0.0, 1.0));

    let sagittarius = Particle {
        mass: 4.297e6,
        position: Vector2{x: 0.00001, y: 0.00001},
        velocity: Vector2{x: 0.0, y: 0.0},
        color: (1.0, 1.0, 1.0),
    };

    let mut milky_way = World::new(settings.clone());
    milky_way.add_world(&bulge_world);
    milky_way.add_world(&thick_disc_world);
    milky_way.add_world(&thick_disc_world);
    milky_way.add_world(&gas_disc1_world);
    milky_way.add_world(&gas_disc2_world);
    milky_way.add_particle(sagittarius);

    milky_way.set_circle_speed(true);

    milky_way
}
