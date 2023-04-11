use crate::scene::Colour;
use std::fs::File;
use std::io::{Result, Write};
use std::ops::{Index, IndexMut};

pub struct Canvas {
    pub pixels: Vec<Vec<Colour>>,
}

impl Canvas {
    pub fn new(columns: usize, rows: usize) -> Self {
        Canvas {
            pixels: vec![vec![Colour::black(); columns]; rows],
        }
    }

    pub fn write_to_file(&self, filename: &str) -> Result<()> {
        let width = self.pixels[0].len();
        let height = self.pixels.len();

        let mut file = File::create(filename)?;
        write!(file, "P3\n{} {}\n255\n", width, height)?;
        for row in &self.pixels {
            for colour in row {
                let red = (colour.red() * 255.0).round() as u8;
                let green = (colour.green() * 255.0).round() as u8;
                let blue = (colour.blue() * 255.0).round() as u8;
                write!(file, "{} {} {} ", red, green, blue)?;
            }
            writeln!(file)?
        }

        Ok(())
    }
}

impl Index<usize> for Canvas {
    type Output = Vec<Colour>;

    fn index(&self, index: usize) -> &Vec<Colour> {
        &self.pixels[index]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::canvas::Canvas;
    use crate::scene::Colour;

    #[test]
    fn test_new_canvas_contains_all_black_pixels() {
        let canvas = Canvas::new(10, 10);

        assert_eq!(canvas.pixels.concat().len(), 100);
        assert_eq!(
            canvas
                .pixels
                .concat()
                .iter()
                .all(|&pixel| pixel == Colour::black()),
            true
        );
    }

    #[test]
    fn test_write_and_read_pixels() {
        let mut canvas = Canvas::new(10, 10);

        canvas[5][5] = Colour::new(1.0, 0.0, 0.0);

        assert_eq!(canvas[5][5], Colour::new(1.0, 0.0, 0.0));
    }
}
