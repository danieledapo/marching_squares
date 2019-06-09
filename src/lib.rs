pub mod simplify;
pub mod svg;

/// A scalar field.
pub trait Field {
    /// Get the width and height of the scalar field.
    fn dimensions(&self) -> (usize, usize);

    /// Calculate the z value at the given position. The position is always inside the range of
    /// `dimensions`.
    fn z_at(&self, x: usize, y: usize) -> f64;

    /// Helper to force a Field to have all the Z values at the boundaries of the field to be set
    /// to `border_z`. Useful to ensure each path is closed.
    fn framed(self, border_z: f64) -> Framed<Self>
    where
        Self: Sized,
    {
        Framed {
            field: self,
            border_z,
        }
    }
}

/// Contours of a shape.
pub type Contours = Vec<Vec<(f64, f64)>>;

/// Find the contours of a given scalar field using `z` as the threshold value.
pub fn march(field: &impl Field, z: f64) -> Contours {
    let (width, height) = field.dimensions();

    let mut segments = vec![];

    let mut current_row_zs = (0..width).map(|x| field.z_at(x, 0)).collect::<Vec<_>>();
    let mut next_row_zs = Vec::with_capacity(width);

    for y in 0..height.saturating_sub(1) {
        next_row_zs.clear();
        next_row_zs.push(field.z_at(0, y + 1));

        for x in 0..width.saturating_sub(1) {
            let ulz = current_row_zs[x];
            let urz = current_row_zs[x + 1];
            let blz = next_row_zs[x];
            let brz = field.z_at(x + 1, y + 1);

            next_row_zs.push(brz);

            let x = x as f64;
            let y = y as f64;

            // TODO: interpolate
            let mx = x + 0.5;
            let my = y + 0.5;

            let mut case = 0;
            if blz > z {
                case |= 1;
            }
            if brz > z {
                case |= 2;
            }
            if urz > z {
                case |= 4;
            }
            if ulz > z {
                case |= 8;
            }

            match case {
                0 | 15 => {}
                1 => {
                    segments.push(((mx, y + 1.0), (x, my)));
                }
                2 => {
                    segments.push(((x + 1.0, my), (mx, y + 1.0)));
                }
                3 => {
                    segments.push(((x + 1.0, my), (x, my)));
                }
                4 => {
                    segments.push(((mx, y), (x + 1.0, my)));
                }
                5 => {
                    segments.push(((mx, y), (x, my)));
                    segments.push(((mx, y + 1.0), (x + 1.0, my)));
                }
                6 => {
                    segments.push(((mx, y), (mx, y + 1.0)));
                }
                7 => {
                    segments.push(((mx, y), (x, my)));
                }
                8 => {
                    segments.push(((x, my), (mx, y)));
                }
                9 => {
                    segments.push(((mx, y + 1.0), (mx, y)));
                }
                10 => {
                    segments.push(((x, my), (mx, y + 1.0)));
                    segments.push(((x + 1.0, my), (mx, y)));
                }
                11 => {
                    segments.push(((x + 1.0, my), (mx, y)));
                }
                12 => {
                    segments.push(((x, my), (x + 1.0, my)));
                }
                13 => {
                    segments.push(((mx, y + 1.0), (x + 1.0, my)));
                }
                14 => {
                    segments.push(((x, my), (mx, y + 1.0)));
                }
                _ => unreachable!(),
            }
        }

        std::mem::swap(&mut current_row_zs, &mut next_row_zs);
    }

    build_contours(segments, (width as f64, height as f64))
}

fn build_contours(mut segments: Vec<((f64, f64), (f64, f64))>, (w, h): (f64, f64)) -> Contours {
    let mut contours = vec![];

    while !segments.is_empty() {
        // prefer to start on a boundary, but if no point lie on a bounday just
        // pick a random one. This allows to connect open paths entirely without
        // breaking them in multiple chunks.
        let first_i = segments
            .iter()
            .enumerate()
            .find(|(_, (s, _))| s.0 == 0.0 || s.0 == w - 1.0 || s.1 == 0.0 || s.1 == h - 1.0)
            .map_or_else(|| segments.len() - 1, |(i, _)| i);

        let first = segments.swap_remove(first_i);
        let mut contour = vec![first.0, first.1];

        loop {
            let prev = contour[contour.len() - 1];
            let next = segments.iter().enumerate().find(|(_, (s, _))| s == &prev);

            match next {
                None => break,
                Some((i, seg)) => {
                    contour.push(seg.1);
                    segments.swap_remove(i);
                }
            }
        }

        contours.push(contour);
    }

    contours
}

#[derive(Debug, Clone)]
pub struct Framed<F> {
    field: F,
    border_z: f64,
}

impl<T: Field> Field for Framed<T> {
    fn dimensions(&self) -> (usize, usize) {
        self.field.dimensions()
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        let (w, h) = self.dimensions();

        if x == 0 || x == w.saturating_sub(1) || y == 0 || y == h.saturating_sub(1) {
            self.border_z + 1e9
        } else {
            self.field.z_at(x, y)
        }
    }
}
