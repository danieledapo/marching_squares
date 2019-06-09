use std::fs::File;
use std::io::Write;

use marching_squares::simplify::simplify;
use marching_squares::svg;
use marching_squares::{march, Field};

#[derive(Debug, Clone)]
struct Fun {
    field: Vec<f64>,
    zrange: (f64, f64),
}

impl Field for Fun {
    fn dimensions(&self) -> (usize, usize) {
        (1600, 1600)
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        let (_w, h) = self.dimensions();
        self.field[y * h + x]
    }
}

impl Fun {
    fn new() -> Self {
        let mut fun = Fun {
            field: Vec::new(),
            zrange: (std::f64::INFINITY, std::f64::NEG_INFINITY),
        };

        let (w, h) = fun.dimensions();
        fun.field.reserve_exact(w * h);

        for y in 0..h {
            for x in 0..w {
                let z = fun.formula(x, y);

                fun.zrange = (fun.zrange.0.min(z), fun.zrange.1.max(z));
                fun.field.push(z);
            }
        }

        fun
    }

    fn formula(&self, x: usize, y: usize) -> f64 {
        let scale = 150.0;

        let (w, h) = self.dimensions();
        let x = (x as f64 - w as f64 / 2.0) / scale;
        let y = (y as f64 - h as f64 / 2.0) / scale;

        (1.3 * x).sin() * (0.9 * y).cos() + (0.8 * x).cos() * (1.9 * y).sin() + (y * 0.2 * x).cos()
    }
}

fn main() {
    let fun = Fun::new();
    let (zmin, zmax) = fun.zrange;

    let c1 = (0xD3, 0x7B, 0x47);
    let c2 = (0x2E, 0x89, 0x72);
    let n = 48;

    let mut nofill_doc = svg::Document::new((0.0, 0.0, 1600.0, 1600.0));
    let mut fill_doc = nofill_doc.clone();

    for i in (0..n).into_iter().rev() {
        let t = f64::from(i) / f64::from(n - 1);
        let z = zmin + (zmax - zmin) * t;

        let contours = march(&fun.clone().framed(z), z);

        let path = svg::Element::path(contours.into_iter().map(|c| simplify(&c)).collect())
            .set("stroke", "black")
            .set("stroke-width", "2");

        nofill_doc = nofill_doc.push(path.clone().fill("none"));
        fill_doc = fill_doc.push(path.fill(lerp_colors(c1, c2, t)));
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
