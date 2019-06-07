use std::fs::File;
use std::io::Write;

use marching_squares::svg;
use marching_squares::{march, Field};

#[derive(Debug, Clone, Copy)]
struct Fun;

impl Field for Fun {
    fn dimensions(&self) -> (usize, usize) {
        (1600, 1600)
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        let scale = 150.0;

        let (w, h) = self.dimensions();
        let x = (x as f64 - w as f64 / 2.0) / scale;
        let y = (y as f64 - h as f64 / 2.0) / scale;

        (1.3 * x).sin() * (0.9 * y).cos() + (0.8 * x).cos() * (1.9 * y).sin() + (y * 0.2 * x).cos()
    }
}

impl Fun {
    fn zrange(&self) -> (f64, f64) {
        let mut zmin = std::f64::INFINITY;
        let mut zmax = std::f64::NEG_INFINITY;

        let (w, h) = self.dimensions();

        for x in 0..w {
            for y in 0..h {
                let z = self.z_at(x, y);

                zmin = zmin.min(z);
                zmax = zmax.max(z);
            }
        }

        (zmin, zmax)
    }
}

fn main() {
    let fun = Fun {};
    let (zmin, zmax) = fun.zrange();

    let c1 = (0xD3, 0x7B, 0x47);
    let c2 = (0x2E, 0x89, 0x72);
    let n = 48;

    let mut nofill_doc = svg::Document::new((0.0, 0.0, 1600.0, 1600.0));
    let mut fill_doc = nofill_doc.clone();

    for i in (0..n).into_iter().rev() {
        let t = f64::from(i) / f64::from(n - 1);
        let z = zmin + (zmax - zmin) * t;
        let countours = march(&fun.framed(z), z);

        for c in countours {
            let poly = svg::Element::polyline(c)
                .set("stroke", "black")
                .set("stroke-width", "3");

            nofill_doc = nofill_doc.push(poly.clone().fill("none"));
            fill_doc = fill_doc.push(poly.fill(lerp_colors(c1, c2, t)));
        }
    }

    for (f, d) in &[
        ("function-no-fill.svg", &nofill_doc),
        ("function-fill.svg", &fill_doc),
    ] {
        let mut f = File::create(f).unwrap();
        write!(f, "{}", d).unwrap();
    }
}

fn lerp_colors(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f64) -> String {
    let r = (f64::from(c1.0) + (f64::from(c2.0) - f64::from(c1.0)) * t) as u8;
    let g = (f64::from(c1.1) + (f64::from(c2.1) - f64::from(c1.1)) * t) as u8;
    let b = (f64::from(c1.2) + (f64::from(c2.2) - f64::from(c1.2)) * t) as u8;

    format!("#{:02x}{:02x}{:02x}", r, g, b)
}
