use crate::color::Color;

#[allow(dead_code)]
pub struct Canvas {
    data: Vec<Vec<Color>>
}

#[allow(dead_code)]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height]
        }
    }

    pub fn data(&self) -> &Vec<Vec<Color>>{
        &self.data
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), &'static str> {

        let width = self.get_width();
        let height = self.get_height();

        if x < width && y < height {
            self.data[y][x] = color;
            Ok(())
        } else {
            Err("Tried to write to a pixel outside the canvas")
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.data = vec![vec![color; self.get_width()]; self.get_height()]
    }

    pub fn get_color(&self, column: usize, row: usize) -> Result<Color, ()>{
        let columns = self.data.len();
        let rows = self.data[0].len();

        if column < columns && row < rows {
            Ok(self.data[row][column])
        } else {
            Err(())
        }
    }

    pub fn get_width(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.data.len()
    }

    pub fn to_ppm(&self) -> String {
        let mut string = String::from("P3\n");
        string.push_str(&String::from(format!("{} {}\n", self.get_width(), self.get_height())));
        string.push_str(&String::from("255\n"));

        let mut char_count = 0;
        for col in &self.data {
            for color in col {
                let r = Self::map_color(color.r()).to_string();
                let g = Self::map_color(color.g()).to_string();
                let b = Self::map_color(color.b()).to_string();

                char_count += r.len() + 1;
                if char_count > 70 {
                    string.push_str("\n");
                    char_count = r.len() + 1;
                }
                string.push_str(&r);
                string.push_str(" ");

                char_count += g.len() + 1;
                if char_count > 70 {
                    string.push_str("\n");
                    char_count = g.len() + 1;
                }
                string.push_str(&g);
                string.push_str(" ");

                char_count += b.len() + 1;
                if char_count > 70 {
                    string.push_str("\n");
                    char_count = b.len() + 1;
                }
                string.push_str(&b);
                string.push_str(" ");
            }
            string.push_str("\n");
            char_count = 0;
        }
        string
    }

    fn map_color(color: f64) -> i64 {
        (color * 255_f64).max(0.0).min(255.0).round() as i64
    }
}
