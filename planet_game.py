# Simple pygame program

# Import and initialize the pygame library
import pygame
import cmath


pygame.init()

# Set up the drawing window
screen = pygame.display.set_mode()

# Run until the user asks to quit
running = True

position = complex(0, 0)
rotation = complex(1, 0)
dist = 80

speed = 1

while running:
    # Did the user click the window close button?
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    keys = pygame.key.get_pressed()
    position += (
        keys[pygame.K_RIGHT]
        - keys[pygame.K_LEFT]
        + 1j * keys[pygame.K_DOWN]
        - 1j * keys[pygame.K_UP]
    ) * speed

    if keys[pygame.K_a]:
        position *= cmath.exp(0.003 * cmath.pi * 1j)
    if keys[pygame.K_d]:
        position *= cmath.exp(-0.003 * cmath.pi * 1j)

    rotation *= cmath.exp(0.01 * cmath.pi * 1j)

    # Fill the background with white
    screen.fill((0, 0, 0))

    # Draw a solid blue circle in the center
    pygame.draw.rect(
        screen,
        (0, 255, 0),
        pygame.Rect(
            position.real + screen.get_width() / 2,
            position.imag + screen.get_height() / 2,
            40,
            40,
        ),
    )

    pygame.draw.rect(
        screen,
        (255, 0, 0),
        pygame.Rect(
            dist * rotation.real + position.real + screen.get_width() / 2,
            dist * rotation.imag + position.imag + screen.get_height() / 2,
            20,
            20,
        ),
    )

    # Flip the display
    pygame.display.flip()

# Done! Time to quit.
pygame.quit()
