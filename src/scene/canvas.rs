use crate::scene::Colour;
use nalgebra::DMatrix;
use std::fs::File;
use std::io::{Result, Write};
use std::ops::{Index, IndexMut};

pub struct Canvas {
    pub pixels: DMatrix<Colour>,
}

impl Canvas {
    pub fn new(columns: usize, rows: usize) -> Self {
        Canvas {
            pixels: DMatrix::from_element(rows, columns, Colour::black()),
        }
    }

    pub fn write_to_file(&self, filename: &str) -> Result<()> {
        let width = self.pixels.ncols();
        let height = self.pixels.nrows();

        let mut file = File::create(filename)?;
        write!(file, "P3\n{} {}\n255\n", width, height)?;
        for row in self.pixels.row_iter() {
            for colour in row.iter() {
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

impl Index<(usize, usize)> for Canvas {
    type Output = Colour;

    fn index(&self, index: (usize, usize)) -> &Colour {
        &self.pixels[index]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
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

        assert_eq!(canvas.pixels.len(), 100);
        assert_eq!(
            canvas.pixels.iter().all(|&pixel| pixel == Colour::black()),
            true
        );
    }

    #[test]
    fn test_write_and_read_pixels() {
        let mut canvas = Canvas::new(10, 10);

        canvas[(5, 5)] = Colour::new(1.0, 0.0, 0.0);

        assert_eq!(canvas[(5, 5)], Colour::new(1.0, 0.0, 0.0));
    }
}
