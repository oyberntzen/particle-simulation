mod simulation;
use std::f64::consts::PI;

use simulation::renderer;

pub fn main() {
    /*let mut world = simulation::World::new();
    world.add_particle(simulation::Particle {
        mass: 1.0,
        position: simulation::Vector2 { x: -1.0, y: 0.0 },
        velocity: simulation::Vector2 { x: 0.0, y: 0.0 },
    });
    world.add_particle(simulation::Particle {
        mass: 1.0,
        position: simulation::Vector2 { x: 1.0, y: 0.0 },
        velocity: simulation::Vector2 { x: 0.0, y: 0.0 },
    });*/

    /*let settings = simulation::WorldSettings {
        gravity_strength: 0.1,
        softening_length: 0.1,
        accuracy: 0.5,
        quadtree: true,
        multiprocessing: true,
    };

    let mut world = simulation::World::new(settings.clone());
    world.new_galaxy(250000, 1.0, 0.7, (1.0, 1.0, 0.0));
    world.add_position(simulation::Vector2 { x: -1.0, y: -0.6 });
    world.add_velocity(simulation::Vector2 { x: 0.2, y: 0.0 });

    let mut world2 = simulation::World::new(settings.clone());
    world2.new_galaxy(250000, 1.0, 0.7, (1.0, 0.0, 1.0));
    world2.add_position(simulation::Vector2 { x: 1.0, y: 0.6 });
    world2.add_velocity(simulation::Vector2 { x: -0.2, y: 0.0 });
    world.add_world(&world2);

    println!("World initialization completed\n");

    let mut renderer = simulation::Renderer::new(1024, 1024);
    let camera = simulation::Camera {
        position: simulation::Vector2 { x: 0.0, y: 0.0 },
        zoom: -2.0,
        brightness: 0.1,
    };

    let frames = 700;
    for i in 0..frames {
        world.update(1.0 / 30.0);
        renderer.render(&world, &camera, i);
        println!("{}/{} frames completed\n", i + 1, frames)
    }*/

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

    let r_max = 50.0;
    let z_max = 10.0;
    let steps_r = 20000;
    let steps_z = 1000;

    let bulge_mass = simulation::distrobution_mass(bulge_density, r_max, z_max, steps_r, steps_z);
    let thin_disc_mass = simulation::distrobution_mass(thin_disc_density, r_max, z_max, steps_r, steps_z);
    let thick_disc_mass = simulation::distrobution_mass(thick_disc_density, r_max, z_max, steps_r, steps_z);
    let gas_disc1_mass = simulation::distrobution_mass(gas_disc1_density, r_max, z_max, steps_r, steps_z);
    let gas_disc2_mass = simulation::distrobution_mass(gas_disc2_density, r_max, z_max, steps_r, steps_z);
    
    let total_mass = bulge_mass + thin_disc_mass + thick_disc_mass + gas_disc1_mass + gas_disc2_mass;
    println!("Total mass: {}", total_mass);

    let total_particles = 500000;
    let bulge_particles = (bulge_mass / total_mass * total_particles as f64) as u32;
    let thin_disc_particles = (thin_disc_mass / total_mass * total_particles as f64) as u32;
    let thick_disc_particles = (thick_disc_mass / total_mass * total_particles as f64) as u32;
    let gas_disc1_particles = (gas_disc1_mass / total_mass * total_particles as f64) as u32;
    let gas_disc2_particles = (gas_disc2_mass / total_mass * total_particles as f64) as u32;

    let mut bulge_world = simulation::from_distrobution(bulge_density, bulge_particles, 25.0, 10.0, 10000, 1000);
    println!("{}", bulge_world.particles.len());

    let mut thin_disc_world = simulation::from_distrobution(thin_disc_density, thin_disc_particles, 25.0, 10.0, 10000, 1000);
    println!("{}", thin_disc_world.particles.len());

    let mut thick_disc_world = simulation::from_distrobution(thick_disc_density, thick_disc_particles, 25.0, 10.0, 10000, 1000);
    println!("{}", thick_disc_world.particles.len());

    let mut gas_disc1_world = simulation::from_distrobution(gas_disc1_density, gas_disc1_particles, 50.0, 10.0, 10000, 1000);
    println!("{}", gas_disc1_world.particles.len());

    let mut gas_disc2_world = simulation::from_distrobution(gas_disc2_density, gas_disc2_particles, 25.0, 10.0, 10000, 1000);
    println!("{}", gas_disc2_world.particles.len());

    let settings = simulation::WorldSettings {
        gravity_strength: 1.30128e-11, //1e20 aar
        softening_length: 0.1,
        accuracy: 0.5,
        quadtree: true,
        multiprocessing: true,
    };


    let mut renderer = simulation::Renderer::new(1024, 1024);
    let camera = simulation::Camera {
        position: simulation::Vector2 { x: 0.0, y: 0.0 },
        zoom: -5.0,
        brightness: 0.2,
    };
    renderer.render(&bulge_world, &camera, "./result/tests/bulge.png");
    renderer.render(&thin_disc_world, &camera, "./result/tests/thin_disk.png");
    renderer.render(&thick_disc_world, &camera, "./result/tests/thick_disk.png");
    renderer.render(&gas_disc1_world, &camera, "./result/tests/gas_disc1.png");
    renderer.render(&gas_disc2_world, &camera, "./result/tests/gas_disc2.png");

    bulge_world.set_color((1.0, 0.0, 0.0));
    thin_disc_world.set_color((0.0, 1.0, 0.0));
    thick_disc_world.set_color((0.0, 1.0, 0.0));
    gas_disc1_world.set_color((0.0, 0.0, 1.0));
    gas_disc2_world.set_color((0.0, 0.0, 1.0));

    let mut milky_way = simulation::World::new(settings);
    milky_way.add_world(&bulge_world);
    milky_way.add_world(&thick_disc_world);
    milky_way.add_world(&thick_disc_world);
    milky_way.add_world(&gas_disc1_world);
    milky_way.add_world(&gas_disc2_world);

    let sun_position = simulation::Vector2{x: 8.21, y: 0.0};
    let sun_gravity = milky_way.calculate_gravity(sun_position);
    let sun_velocity = (sun_gravity.abs() * sun_position.abs()).sqrt();
    let sun_period = sun_position.abs() * 2.0 * PI / sun_velocity;
    println!("Sun period: {}", sun_period);
    let dt = sun_period / (30.0 * 60.0);

    renderer.render(&milky_way, &camera, "./result/tests/milky_way.png");

    milky_way.set_circle_speed();
    let frames = 1000000;
    for i in 0..frames {
        milky_way.update(dt);
        renderer.render(&milky_way, &camera, format!("./result/frames/{:05}.png", i).as_str());
        println!("{}/{} frames completed", i + 1, frames);

        if i % 100 == 0 {
            println!("Saving particles");
            milky_way.save_to_file(format!("./result/particles/particles{:05}", i).as_str());
        }
        println!();
    }
}
