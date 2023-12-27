use super::*;

pub struct Camera {
    pub position: Vector2,
    pub zoom: f64,
    pub brightness: f64,
}

pub struct Renderer {
    width: u32,
    height: u32,
    img_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    color_buffer: Vec<(f64, f64, f64)>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut color_buffer = Vec::new();
        color_buffer.resize((width * height) as usize, (0.0, 0.0, 0.0));
        Self {
            width,
            height,
            img_buffer: image::ImageBuffer::new(width, height),
            color_buffer,
        }
    }

    pub fn render(&mut self, world: &World, camera: &Camera, frame: u32) {
        self.img_buffer.fill(0);
        self.color_buffer.fill((0.0, 0.0, 0.0));

        for particle in &world.particles {
            let screen_pos = self.vector_world_to_screen(particle.position, camera);
            if screen_pos.x < 0.0
                || screen_pos.x >= self.width as f64
                || screen_pos.y < 0.0
                || screen_pos.y >= self.height as f64
            {
                continue;
            }
            let x = screen_pos.x as usize;
            let y = screen_pos.y as usize;
            let i = y * (self.height as usize) + x;

            self.color_buffer[i].0 += particle.color.0;
            self.color_buffer[i].1 += particle.color.1;
            self.color_buffer[i].2 += particle.color.2;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y * self.height + x) as usize;

                let r = (self.color_buffer[i].0 * camera.brightness * 255.0) as u8;
                let g = (self.color_buffer[i].1 * camera.brightness * 255.0) as u8;
                let b = (self.color_buffer[i].2 * camera.brightness * 255.0) as u8;
                self.img_buffer
                    .put_pixel(x, y, image::Rgb::<u8> { 0: [r, g, b] });
            }
        }

        self.img_buffer
            .save(format!("result/frames/{:05}.png", frame))
            .unwrap();
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
