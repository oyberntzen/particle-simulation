import pygame
import random

def main():
    pygame.init()

    screen = pygame.display.set_mode([500, 500])
    counter = 0
    while True:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                quit()
        

        screen.fill((0, 0, 0))

        for i in range(1_000_000):
            x = int(500*random.random())
            y = int(500*random.random())
            screen.set_at((x, y), (255, 255, 255))

        pygame.display.update()
        print(f"Frame: {counter}")
        counter += 1

if __name__ == "__main__":
    main()