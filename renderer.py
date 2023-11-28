import pygame
import math
from vector import Vector2

class Renderer:
    def __init__(self, width, height, fps):
        self.width = width
        self.height = height
        self.fps = fps

        pygame.init()
        self.screen = pygame.display.set_mode([width, height])
        self.clock = pygame.time.Clock()

        self.cam_pos = Vector2(0, 0)
        self.zoom = 0

        self.particle_size = 0.05

    def update(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return True
            elif event.type == pygame.MOUSEWHEEL:
                self.zoom += event.y * 0.1
            elif event.type == pygame.MOUSEMOTION:
                if event.buttons[0] == 1:
                    self.cam_pos.x -= self.size_screen_to_world(event.rel[0])
                    self.cam_pos.y += self.size_screen_to_world(event.rel[1])
                
        return False
    
    def draw(self, world):
        self.screen.fill((0, 0, 0))
        for particle in world.particles:
            pos = self.vector_world_to_screen(particle.pos)
            radius = max(self.size_world_to_screen(self.particle_size*math.sqrt(particle.mass)), 1)
            pygame.draw.circle(self.screen, (255, 255, 255), (pos.x, pos.y), radius)
        pygame.display.update()
        self.clock.tick(self.fps)
        

    def vector_world_to_screen(self, pos):
        #print(pos)
        new_pos = pos - self.cam_pos
        #print(new_pos)
        new_pos.y = -new_pos.y
        #print(new_pos)
        new_pos *= (2**self.zoom) * (self.width/2)
        #print(new_pos)
        new_pos += Vector2(self.width/2, self.height/2)
        #print(new_pos)
        return new_pos

    def size_world_to_screen(self, size):
        return size * (2**self.zoom) * (self.width/2)


    def size_screen_to_world(self, size):
        return size / ((2**self.zoom) * (self.width/2))