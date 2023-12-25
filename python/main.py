from vector import Vector2
from particle import Particle
from world import World
from renderer import Renderer
import time
import random
import math

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

    days_per_second = 1 
    while True:
        world.update(60*24*days_per_second)
        if renderer.update():
            break
        renderer.draw(world)

def particle_square():
    world = World()
    world.softening_length = 0.1
    n = 100 

    for i in range(n):
        world.add_particle(Particle(1/n, Vector2(random.random()*2-1, random.random()*2-1), Vector2(0, 0)))


    renderer = Renderer(500, 500, 60)

    while True:
        world.update(1/60)
        if renderer.update():
            break
        renderer.draw(world)
    

def generate_galaxy(n, max_r, mass, color, gravity_strength=0.1):
    world = World()
    world.softening_length = 0.1
    world.gravity_strength = gravity_strength

    for i in range(n):
        r = random.random()*max_r
        angle = random.random() * 2*math.pi
        pos = Vector2(r*math.cos(angle), r*math.sin(angle))
        world.add_particle(Particle(1/n, pos, Vector2(0, 0), color=color))

    world.set_circle_speed(Vector2(0, 0))
    return world

def galaxy_collision():
    world2 = generate_galaxy(50, 0.3, 0.5, (1, 0, 0), 0.1)
    world = generate_galaxy(100, 0.5, 1, (0, 0, 1), 0.1)
    world.merge_with(world2, Vector2(1, 1), Vector2(-0.2, 0))

    renderer = Renderer(500, 500, 60)

    while True:
        world.update(1/60)
        if renderer.update():
            break
        renderer.draw(world)



def main():
    galaxy_collision()

if __name__ == "__main__":
    main()