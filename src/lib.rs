use std::collections::HashMap;

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

/// A `SegmentsMap` is used to speedup contour building on the average case. It's simply a map from
/// the start position of the segment rounded with integers coordinates to the list of all the
/// segments that start in that position. Usually, shapes have very few segments that start at the
/// same integer position thus this simple optimization allows to find the next segment in O(1)
/// which is great.
///
/// Note that a valid `SegmentsMap` must not have entries for an empty list of segments.
type SegmentsMap = HashMap<(u64, u64), Vec<((f64, f64), (f64, f64))>>;

/// Find the contours of a given scalar field using `z` as the threshold value.
pub fn march(field: &impl Field, z: f64) -> Contours {
    let (width, height) = field.dimensions();

    let mut segments: SegmentsMap = HashMap::new();
    let mut add_seg = |s: (f64, f64), e| {
        segments
            .entry((s.0 as u64, s.1 as u64))
            .or_default()
            .push((s, e));
    };

    // avoid calling z_at multiple times for the same cell by storing the z values for the current
    // row and by storing the values for the next row as soon as they're calculated.
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

            // if we're at the boundary of the image, consider solid squares as
            // borders in order to close paths
            case = match case {
                15 => {
                    if x == 0 {
                        6
                    } else if x == width - 2 {
                        9
                    } else if y == 0 {
                        3
                    } else if y == height - 2 {
                        12
                    } else {
                        case
                    }
                }
                _ => case,
            };

            let x = x as f64;
            let y = y as f64;

            // TODO: interpolate
            let mx = x + 0.5;
            let my = y + 0.5;

            match case {
                0 | 15 => {}
                1 => {
                    add_seg((mx, y + 1.0), (x, my));
                }
                2 => {
                    add_seg((x + 1.0, my), (mx, y + 1.0));
                }
                3 => {
                    add_seg((x + 1.0, my), (x, my));
                }
                4 => {
                    add_seg((mx, y), (x + 1.0, my));
                }
                5 => {
                    add_seg((mx, y), (x, my));
                    add_seg((mx, y + 1.0), (x + 1.0, my));
                }
                6 => {
                    add_seg((mx, y), (mx, y + 1.0));
                }
                7 => {
                    add_seg((mx, y), (x, my));
                }
                8 => {
                    add_seg((x, my), (mx, y));
                }
                9 => {
                    add_seg((mx, y + 1.0), (mx, y));
                }
                10 => {
                    add_seg((x, my), (mx, y + 1.0));
                    add_seg((x + 1.0, my), (mx, y));
                }
                11 => {
                    add_seg((x + 1.0, my), (mx, y));
                }
                12 => {
                    add_seg((x, my), (x + 1.0, my));
                }
                13 => {
                    add_seg((mx, y + 1.0), (x + 1.0, my));
                }
                14 => {
                    add_seg((x, my), (mx, y + 1.0));
                }
                _ => unreachable!(),
            }
        }

        std::mem::swap(&mut current_row_zs, &mut next_row_zs);
    }

    build_contours(segments, (width as u64, height as u64))
}

fn build_contours(mut segments: SegmentsMap, (w, h): (u64, u64)) -> Contours {
    use std::collections::hash_map::Entry;

    let mut contours = vec![];

    while !segments.is_empty() {
        // prefer to start on a boundary, but if no point lie on a bounday just
        // pick a random one. This allows to connect open paths entirely without
        // breaking them in multiple chunks.
        let first_k = segments
            .iter()
            .find(|(s, _)| s.0 == 0 || s.0 == w - 1 || s.1 == 0 || s.1 == h - 1)
            .map_or_else(|| segments.keys().next().unwrap(), |(k, _)| k)
            .clone();

        let mut first_e = match segments.entry(first_k) {
            Entry::Occupied(o) => o,
            Entry::Vacant(_) => unreachable!(),
        };

        let first = first_e.get_mut().pop().unwrap();
        if first_e.get().is_empty() {
            first_e.remove_entry();
        }

        let mut contour = vec![first.0, first.1];

        loop {
            let prev = contour[contour.len() - 1];

            let mut segments = match segments.entry((prev.0 as u64, prev.1 as u64)) {
                Entry::Vacant(_) => break,
                Entry::Occupied(o) => o,
            };

            let next = segments
                .get()
                .iter()
                .enumerate()
                .find(|(_, (s, _))| s == &prev);

            match next {
                None => break,
                Some((i, seg)) => {
                    contour.push(seg.1);

                    segments.get_mut().swap_remove(i);
                    if segments.get().is_empty() {
                        segments.remove_entry();
                    }
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
            self.border_z + 1e-9
        } else {
            self.field.z_at(x, y)
        }
    }
}
