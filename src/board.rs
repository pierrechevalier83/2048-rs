extern crate matrix_display;
use self::matrix_display::*;

pub struct Board {
    format: Format,
    colour_theme: [i32; 17],
}

impl Board {
    pub fn new() -> Board {
        let colour_theme = [0, 247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54,
                            53, 52];

        Board {
            format: Format::new(7, 3),
            colour_theme: colour_theme,
        }
    }
    fn cells(&self, data: [i32; 16]) -> Vec<cell::Cell<String>> {
        data.iter()
            .cloned()
            .map(|i| (2_f64.powi(i), *self.colour_theme.get(i as usize).unwrap() as u8))
            .map(|(x, col)| match x {
                1_f64 => (".".to_string(), col),
                _ => (x.to_string(), col),
            })
            .map(|(s, col)| cell::Cell::new(s, 7, col))
            .collect::<Vec<_>>()
    }
    pub fn print(self, data: [i32; 16]) {
        let matrix = matrix::Matrix::new(4, self.cells(data));
        let display = MatrixDisplay::new(self.format, matrix);
        display.print(&mut ::std::io::stdout(), &style::BordersStyle::Thick);
    }
}
