from vector import Vector2

class Particle:
    def __init__(self, mass=1, pos=Vector2(0,0), vel=Vector2(0,0)):
        self.mass = mass
        self.pos = pos
        self.vel = vel
    
    def update(self, force, dt):
        acceleration = force / self.mass
        self.pos += self.vel * dt
        self.vel += acceleration * dt