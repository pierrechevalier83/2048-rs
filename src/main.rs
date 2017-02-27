extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(7, 3);
    let colour_theme = vec![247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54, 53, 52];
    let board = (0..16)
        .map(|x| {
            cell::Cell::new(2_f64.powi(x + 1),
                      7,
                      *colour_theme.get(x as usize).unwrap() as u8)
        })
        .collect::<Vec<_>>();
    let data = matrix::Matrix::new(4, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::Thick);
}
