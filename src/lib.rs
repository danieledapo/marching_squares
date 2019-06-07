pub mod svg;

pub trait Field {
    fn dimensions(&self) -> (usize, usize);
    fn z_at(&self, x: usize, y: usize) -> f64;
}

pub type Countours = Vec<Vec<(f64, f64)>>;

pub fn march(field: &impl Field, z: f64) -> Countours {
    let (width, height) = field.dimensions();

    let mut segments = vec![];

    for y in 0..height.saturating_sub(1) {
        for x in 0..width.saturating_sub(1) {
            let ulz = field.z_at(x, y);
            let urz = field.z_at(x + 1, y);
            let blz = field.z_at(x, y + 1);
            let brz = field.z_at(x + 1, y + 1);

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
    }

    build_countours(segments, (width as f64, height as f64))
}

fn build_countours(mut segments: Vec<((f64, f64), (f64, f64))>, (w, h): (f64, f64)) -> Countours {
    // TODO: make it more efficient

    let mut countours = vec![];

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
        let mut countour = vec![first.0, first.1];

        loop {
            let prev = countour[countour.len() - 1];
            let next = segments.iter().enumerate().find(|(_, (s, _))| s == &prev);

            match next {
                None => break,
                Some((i, seg)) => {
                    countour.push(seg.1);
                    segments.swap_remove(i);
                }
            }
        }

        countours.push(countour);
    }

    countours
}
