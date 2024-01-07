mod simulation;
use std::f64::consts::PI;

pub fn main() {
    last_image_galaxy_collision();
}

fn single_galaxy() {
    let mut milkyway = simulation::milkyway();

    let mut renderer = simulation::Renderer::new(1024, 1024);
    let camera = simulation::Camera {
        position: simulation::Vector2 { x: 0.0, y: 0.0 },
        zoom: -5.0,
        brightness: 0.3,
    };
    renderer.render(&milkyway, &camera, "./result/tests/milkyway.png");

    milkyway.save_to_file("./result/tests/milkyway.bin");

    let frames = 1000000;
    for i in 0..frames {
        milkyway.update(1.0);
        renderer.render(&milkyway, &camera, format!("./result/frames/{:05}.png", i).as_str());
        println!("{}/{} frames completed", i + 1, frames);

        if i % 100 == 0 {
            println!("Saving particles");
            milkyway.save_to_file(format!("./result/particles/particles{:05}.bin", i).as_str());
        }
        println!();
    }
}

fn galaxy_collision() {
    let settings = simulation::WorldSettings {
        gravity_strength: 1.30128e-12, //1e20 aar
        softening_length: 0.1,
        accuracy: 0.5,
        quadtree: true,
        multiprocessing: true,
    };

    let mut milkyway1 = simulation::World::new(settings.clone());
    milkyway1.load_from_file("./result/tests/milkyway_1300.bin");

    let mut milkyway2 = simulation::World::new(settings);
    milkyway2.load_from_file("./result/tests/milkyway_1300.bin");

    milkyway1.add_position(simulation::Vector2 { x: -25.0, y: -25.0 });
    milkyway2.add_position(simulation::Vector2 { x: 25.0, y: 25.0 });

    milkyway1.add_velocity(simulation::Vector2 { x: 0.03, y: 0.03 });
    milkyway2.add_velocity(simulation::Vector2 { x: -0.03, y: -0.03 });

    milkyway1.add_world(&milkyway2);
    let mut world = milkyway1;

    let mut renderer = simulation::Renderer::new(1024, 1024);
    let camera = simulation::Camera {
        position: simulation::Vector2 { x: 0.0, y: 0.0 },
        zoom: -6.0,
        brightness: 0.3,
    };
    renderer.render(&world, &camera, "./result/tests/collision.png");

    let frames = 1000000;
    for i in 0..frames {
        world.update(1.0);
        renderer.render(&world, &camera, format!("./result/frames/{:05}.png", i).as_str());
        println!("{}/{} frames completed", i + 1, frames);

        if i % 100 == 0 {
            println!("Saving particles");
            world.save_to_file(format!("./result/particles/particles{:05}.bin", i).as_str());
        }
        println!();
    }
}

fn continue_galaxy_collision() {
    let settings = simulation::WorldSettings {
        gravity_strength: 1.30128e-12, //1e20 aar
        softening_length: 0.1,
        accuracy: 0.5,
        quadtree: true,
        multiprocessing: true,
    };

    let mut world = simulation::World::new(settings);
    world.load_from_file("./result/particles/particles02300.bin");

    let mut renderer = simulation::Renderer::new(1024, 1024);
    let camera = simulation::Camera {
        position: simulation::Vector2 { x: 0.0, y: 0.0 },
        zoom: -6.0,
        brightness: 0.3,
    };

    let frames = 1000000;
    for i in 2301..frames {
        world.update(4.0);
        renderer.render(&world, &camera, format!("./result/frames/{:05}.png", i).as_str());
        println!("{}/{} frames completed", i + 1, frames);

        if i % 100 == 0 {
            println!("Saving particles");
            world.save_to_file(format!("./result/particles/particles{:05}.bin", i).as_str());
        }
        println!();
    }
}

fn last_image_galaxy_collision() {
    let settings = simulation::WorldSettings {
        gravity_strength: 1.30128e-12, //1e20 aar
        softening_length: 0.1,
        accuracy: 0.5,
        quadtree: true,
        multiprocessing: true,
    };

    let mut world = simulation::World::new(settings);
    world.load_from_file("./result/particles/particles02700.bin");

    let mut renderer = simulation::Renderer::new(3840, 2160);
    let camera = simulation::Camera {
        position: simulation::Vector2 { x: 0.0, y: 0.0 },
        zoom: -9.5,
        brightness: 0.3,
    };

    renderer.render(&world, &camera, "result/tests/collision_after.png");
}