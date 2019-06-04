pub mod svg;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait Field {
    fn dimensions(&self) -> (usize, usize);
    fn z_at(&self, x: usize, y: usize) -> f64;
}

pub type Countours = Vec<Vec<Point>>;

pub fn march(field: &impl Field, z: f64) -> Countours {
    let (width, height) = field.dimensions();

    let mut countours = vec![];

    for y in 0..height.saturating_sub(1) {
        for x in 0..width.saturating_sub(1) {
            let ul = field.z_at(x, y);
            let ur = field.z_at(x + 1, y);
            let bl = field.z_at(x, y + 1);
            let br = field.z_at(x + 1, y + 1);

            let mut case = 0;
            if ul > z {
                case |= 1;
            }
            if ur > z {
                case |= 2;
            }
            if bl > z {
                case |= 4;
            }
            if br > z {
                case |= 8;
            }

            if case == 0 || case == 15 {
                continue;
            }
        }
    }

    countours
}
