from vector import Vector2
from particle import Particle
from world import World
from renderer import Renderer
import time
import random

def test():
    world = World()
    world.add_particle(Particle(2, Vector2(-1, 0), Vector2(0, 0.1)))
    world.add_particle(Particle(1, Vector2(1, 0), Vector2(0, -0.1)))

    renderer = Renderer(500, 500, 60)

    while True:
        world.update(1/60)
        if renderer.update():
            break
        renderer.draw(world)

def solar_system():
    world = World()
    world.gravity_strength = 6.67e-11

    world.add_particle(Particle(1.98e30, Vector2(0, 0), Vector2(0, 0))) #Sun
    world.add_particle(Particle(5.972e24, Vector2(1.5e11, 0), Vector2(0, 2.978e4))) #Earth

    renderer = Renderer(500, 500, 60)
    renderer.zoom = -38
    renderer.particle_size = 1e-5

    days_per_second = 30 
    while True:
        world.update(60*24*days_per_second)
        if renderer.update():
            break
        renderer.draw(world)

def particle_square():
    world = World()
    world.softening_length = 0.1
    n = 200 

    for i in range(n):
        world.add_particle(Particle(1/n, Vector2(random.random()*2-1, random.random()*2-1), Vector2(0, 0)))

    renderer = Renderer(500, 500, 60)

    while True:
        world.update(1/60)
        if renderer.update():
            break
        renderer.draw(world)

def main():
    particle_square()

if __name__ == "__main__":
    main()