use super::*;

pub struct Camera {
    pub position: Vector2,
    pub zoom: f64
}

pub struct Renderer {
    buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: image::ImageBuffer::new(width, height),
            width, height
        }
    }

    pub fn render(&mut self, world: &World, camera: &Camera, frame: u32) {
        self.buffer.fill(0);

        for particle in &world.particles {
            let screen_pos = self.vector_world_to_screen(particle.position, camera);
            if screen_pos.x < 0.0 || screen_pos.x >= self.width as f64 || screen_pos.y < 0.0 || screen_pos.y >= self.height as f64 {
                continue;
            }
            let x = screen_pos.x as u32;
            let y = screen_pos.y as u32;
            self.buffer.put_pixel(x, y, image::Rgb::<u8>{0: [255; 3]});
        }
        self.buffer.save(format!("result/frames/{:05}.png", frame)).unwrap();
    }

    fn vector_world_to_screen(&self, vector: Vector2, camera: &Camera) -> Vector2 {
        let mut new_vector = vector - camera.position;
        new_vector.y = -new_vector.y;
        new_vector *= 2f64.powf(camera.zoom) * (self.width as f64 / 2.0);
        new_vector.x += self.width as f64 / 2.0;
        new_vector.y += self.height as f64 / 2.0;

        new_vector
    }
}
