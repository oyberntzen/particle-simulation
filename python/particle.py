from vector import Vector2

class Particle:
    def __init__(self, mass=1, pos=Vector2(0,0), vel=Vector2(0,0), color=(1,1,1)):
        self.mass = mass
        self.pos = pos
        self.vel = vel
        self.color = color
    
    def update(self, force, dt):
        acceleration = force / self.mass
        self.vel += acceleration * dt
        self.pos += self.vel * dt