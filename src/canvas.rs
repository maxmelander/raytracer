use crate::color::Color;

#[allow(dead_code)]
pub struct Canvas {
    data: Vec<Vec<Color>>
}

#[allow(dead_code)]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![Color::new(0.0, 0.0, 0.0); height]; width]
        }
    }

    pub fn data(&self) -> &Vec<Vec<Color>>{
        &self.data
    }

    pub fn write_pixel(&mut self, column: usize, row: usize, color: Color) {
        self.data[column][row] = color;
    }

    pub fn get_color(&self, column: usize, row: usize) -> Result<Color, ()>{
        let columns = self.data.len();
        let rows = self.data[0].len();

        if column < columns && row < rows {
            Ok(self.data[column][row])
        } else {
            Err(())
        }
    }
}
