use super::matrix::Matrix4;
use super::ray::Ray;
use super::tuple::Tuple;
use super::world::World;
use super::canvas::Canvas;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub h_size: usize,
    pub v_size: usize,
    pub field_of_view: f64,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub transform: Matrix4,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(h_size: usize, v_size: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (h_size as f64) / (v_size as f64);

        let (half_width, half_height) = if aspect >= 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.) / h_size as f64;

        Self {
            h_size,
            v_size,
            field_of_view,
            pixel_size,
            half_width,
            half_height,
            transform: Matrix4::new_identity(),
        }
    }

    pub fn ray_for_pixel(self, px: usize, py: usize) -> Result<Ray, &'static str> {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        if let Some(transform_inverse) = self.transform.inverse() {
            let pixel = transform_inverse * Tuple::new_point(world_x, world_y, -1.);
            let origin = transform_inverse * Tuple::new_point(0., 0., 0.);
            let direction = (pixel - origin).normalize();
            return Ray::new(origin, direction);
        }
        Err("Could not create a ray")
    }

    pub fn render(self, world: &World) -> Result<Canvas, &'static str> {
        let mut canvas = Canvas::new(self.h_size, self.v_size);

        for y in 0..self.v_size - 1 {
            for x in 0..self.h_size - 1 {
                let ray = self.ray_for_pixel(x, y)?;
                let color = world.color_at(ray);
                canvas.write_pixel(x, y, color)?;
            }
        }
        Ok(canvas)
    }
}
