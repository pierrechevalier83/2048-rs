use std::borrow::Cow;

use matrix_display::*;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

#[derive(Clone)]
pub struct Board {
    colour_theme: [i32; 17],
    rng: Xoshiro256Plus,
}

impl Board {
    pub fn new() -> Board {
        let colour_theme = [
            0, 247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54, 53, 52,
        ];

        Board {
            colour_theme,
            rng: Xoshiro256Plus::from_entropy(),
        }
    }
    pub fn print<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let mut matrix = matrix::Matrix::new(
            4,
            data.into_iter()
                .map(|i| {
                    (
                        2i32.pow(i as u32),
                        *self.colour_theme.get(i as usize).unwrap() as u8,
                    )
                })
                .map(|(x, col)| {
                    (
                        if x == 1 {
                            Cow::Borrowed(".")
                        } else {
                            Cow::Owned(x.to_string())
                        },
                        col,
                    )
                })
                .map(|(s, col)| cell::Cell::new(s, 0, col))
                .collect::<Vec<_>>(),
        );
        const FORMAT: Format = Format {
            cell_w: 7,
            cell_h: 3,
        };
        let display = MatrixDisplay::new(&FORMAT, &mut matrix);
        display.print(out, &style::BordersStyle::Heavy);
    }
    pub fn print_inactive<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let mut grey_scale = self.clone();
        grey_scale.colour_theme = [
            0, 255, 251, 248, 246, 244, 242, 241, 240, 239, 238, 237, 236, 235, 234, 233, 232,
        ];
        grey_scale.print(data, out);
    }
    pub fn print_lost<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let mut red_scale = self.clone();
        red_scale.colour_theme = [
            0, 90, 126, 162, 198, 197, 161, 125, 89, 53, 17, 196, 160, 124, 88, 52, 16,
        ];
        red_scale.print(data, out);
    }
    pub fn print_won<W>(&self, data: [i32; 16], out: &mut W)
    where
        W: ::std::io::Write,
    {
        let mut fireworks = self.clone();
        let mut fw: [i32; 17] = [0; 17];
        fw.iter_mut()
            .for_each(|f| *f = fireworks.rng.gen_range(1..256));
        fw[0] = 0;
        fireworks.colour_theme[..].clone_from_slice(&fw[..]);
        fireworks.print(data, out);
    }
}
