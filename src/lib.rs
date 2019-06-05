pub mod svg;

pub trait Field {
    fn dimensions(&self) -> (usize, usize);
    fn z_at(&self, x: usize, y: usize) -> f64;
}

pub type Countours = Vec<Vec<(f64, f64)>>;

pub fn march(field: &impl Field, z: f64) -> Countours {
    let (width, height) = field.dimensions();

    let mut countours = vec![];

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
            if ulz > z {
                case |= 1;
            }
            if urz > z {
                case |= 2;
            }
            if brz > z {
                case |= 4;
            }
            if blz > z {
                case |= 8;
            }

            match case {
                0 | 15 => {}
                1 => {
                    countours.push(((mx, y), (x, my)));
                }
                2 => {
                    countours.push(((mx, y), (x + 1.0, my)));
                }
                3 => {
                    countours.push(((x, y), (x + 1.0, y)));
                }
                4 => {
                    countours.push(((x + 1.0, my), (mx, y + 1.0)));
                }
                5 => {
                    // TODO: saddle
                }
                6 => {
                    countours.push(((x + 1.0, y), (x + 1.0, y + 1.0)));
                }
                7 => {
                    countours.push(((x, my), (mx, y + 1.0)));
                }
                8 => {
                    countours.push(((mx, y + 1.0), (x, my)));
                }
                9 => {
                    countours.push(((x, y + 1.0), (x, y)));
                }
                10 => {
                    // TODO: saddle
                }
                11 => {
                    countours.push(((x + 1.0, my), (mx, y + 1.0)));
                }
                12 => {
                    countours.push(((x + 1.0, y + 1.0), (x, y + 1.0)));
                }
                13 => {
                    countours.push(((mx, y), (x + 1.0, my)));
                }
                14 => {
                    countours.push(((mx, y), (x, my)));
                }
                _ => unreachable!(),
            }
        }
    }

    // TODO: build actual countours from segments
    countours.into_iter().map(|c| vec![c.0, c.1]).collect()
}
