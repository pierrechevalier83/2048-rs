extern crate matrix_display;
use self::matrix_display::*;

pub struct Board {
    format: Format,
    matrix: matrix::Matrix<cell::Cell<i32>>,
}

impl Board {
    pub fn new() -> Board {
        let colour_theme = vec![247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54, 53, 52];
        let board = (0..16)
            .map(|x| {
                cell::Cell::new(2_f64.powi(x + 1) as i32,
                          7,
                          *colour_theme.get(x as usize).unwrap() as u8)
            })
            .collect::<Vec<_>>();
	    Board {
            format: Format::new(7, 3),
		    matrix: matrix::Matrix::new(4, board),
		}
	}
	pub fn print(self) {
        let display = MatrixDisplay::new(self.format, self.matrix);
        display.print(&mut ::std::io::stdout(), &style::BordersStyle::Thick);
	}
}
