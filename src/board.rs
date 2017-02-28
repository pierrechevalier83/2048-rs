extern crate matrix_display;
use self::matrix_display::*;

pub struct Board {
    format: Format,
    colour_theme: [i32; 16],
}

impl Board {
    pub fn new() -> Board {
        let colour_theme = [247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54, 53,
                            52];

        Board {
            format: Format::new(7, 3),
            colour_theme: colour_theme,
        }
    }
    fn cells(&self, data: ::std::ops::Range<i32>) -> Vec<cell::Cell<i32>> {
        data.map(|x| {
                cell::Cell::new(2_f64.powi(x + 1) as i32,
                                7,
                                *self.colour_theme.get(x as usize).unwrap() as u8)
            })
            .collect::<Vec<_>>()
    }
    pub fn print(self, data: ::std::ops::Range<i32>) {
        let matrix = matrix::Matrix::new(4, self.cells(data));
        let display = MatrixDisplay::new(self.format, matrix);
        display.print(&mut ::std::io::stdout(), &style::BordersStyle::Thick);
    }
}
