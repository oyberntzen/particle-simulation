from particle import Particle
from vector import Vector2

class World:
    def __init__(self):
        self.particles = []
        self.gravity_strength = 0.1 
        self.softening_length = 0.1
    
    def add_particle(self, particle):
        self.particles.append(particle)

    def update(self, dt):
        forces = self.calculate_gravity()

        for particle, force in zip(self.particles, forces):
            particle.update(force, dt)

    def calculate_gravity(self):
        forces = []
        for particle in self.particles:
            force = Vector2(0, 0)

            for other in self.particles:
                if particle == other:
                    continue
                distance = abs(other.pos-particle.pos)
                magnitude = self.gravity_strength * particle.mass*other.mass / (distance*distance + self.softening_length*self.softening_length)
                unit_vector = (other.pos-particle.pos)/distance
                force += unit_vector * magnitude
            forces.append(force)

        return forces
