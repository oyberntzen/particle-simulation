use std::fs;

mod simulation;

pub fn main() {
    let mut world = simulation::World::new();
    world.add_particle(simulation::Particle { mass: 1.0, position: simulation::Vector2 { x: -1.0, y: 0.0 }, velocity: simulation::Vector2 { x: 0.0, y: 0.0 } });
    world.add_particle(simulation::Particle { mass: 1.0, position: simulation::Vector2 { x: 1.0, y: 0.0 }, velocity: simulation::Vector2 { x: 0.0, y: 0.0 } });

    let mut file = fs::File::create("data/frames/simulation1.sim").unwrap();

    for _ in 0..1000000 {
        world.update(0.1);
        let _ = world.write_to_file(&mut file);
    }
}
