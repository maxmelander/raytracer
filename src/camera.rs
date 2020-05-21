use super::matrix::Matrix4;

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

impl Camera {
    pub fn new(h_size: usize, v_size: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (h_size as f64) / (v_size as f64);

        let mut half_width = 0.;
        let mut half_height = 0.;

        if aspect >= 1 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

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
}
