mod simulation;

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

    let settings = simulation::WorldSettings {
        gravity_strength: 0.1,
        softening_length: 0.1,
        accuracy: 0.5,
        quadtree: true,
        multiprocessing: true,
    };
    
    let mut world = simulation::World::new(settings.clone());
    world.new_galaxy(250000, 1.0, 0.7, (255, 255, 0));
    world.add_position(simulation::Vector2 { x: -1.0, y: -0.6 });
    world.add_velocity(simulation::Vector2 { x: 0.2, y: 0.0 });

    let mut world2 = simulation::World::new(settings.clone());
    world2.new_galaxy(250000, 1.0, 0.7, (255, 0, 255));
    world2.add_position(simulation::Vector2 { x: 1.0, y: 0.6 });
    world2.add_velocity(simulation::Vector2 { x: -0.2, y: 0.0 });
    world.add_world(&world2);

    println!("World initialization completed\n");

    let mut renderer = simulation::Renderer::new(1024, 1024);
    let camera = simulation::Camera {
        position: simulation::Vector2 { x: 0.0, y: 0.0 },
        zoom: -2.0,
    };

    let frames = 700;
    for i in 0..frames {
        world.update(1.0 / 30.0);
        renderer.render(&world, &camera, i);
        println!("{}/{} frames completed\n", i + 1, frames)
    }
}
