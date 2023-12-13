mod simulation;

pub fn main() {
    let mut world = simulation::World::new();
    world.add_particle(simulation::Particle { mass: 1.0, position: simulation::Vector2 { x: -1.0, y: 0.0 }, velocity: simulation::Vector2 { x: 0.0, y: 0.0 } });
    world.add_particle(simulation::Particle { mass: 1.0, position: simulation::Vector2 { x: 1.0, y: 0.0 }, velocity: simulation::Vector2 { x: 0.0, y: 0.0 } });


    let mut renderer = simulation::Renderer::new(500, 500);
    let camera = simulation::Camera{position: simulation::Vector2 { x: 0.0, y: 0.0 }, zoom: 0.0};

    let frames = 1000;
    for i in 0..frames {
        world.update(1.0/30.0);
        renderer.render(&world, &camera, i);
        println!("{}/{} frames completed", i+1, frames)
    }
}
