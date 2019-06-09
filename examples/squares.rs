use std::fs::File;
use std::io::Write;

use marching_squares::simplify::simplify;
use marching_squares::svg;
use marching_squares::{march, Field};

#[derive(Debug, Clone)]
struct Example;

impl Field for Example {
    fn dimensions(&self) -> (usize, usize) {
        (200, 200)
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        // square
        let square = 10..=50;
        if square.contains(&x) && square.contains(&y) {
            let subsquare = 20..=40;
            if subsquare.contains(&x) && subsquare.contains(&y) {
                return 0.0;
            }

            return 1.0;
        }

        // rotated square
        let c = 100;
        let dx = x.max(c) - x.min(c);
        let dy = y.max(c) - y.min(c);
        if dx + dy < 50 {
            if dx + dy < 49 {
                return 0.0;
            }

            return 1.0;
        }

        return 0.0;
    }
}

fn main() {
    let contours = march(&Example {}, 0.5);

    let mut wireframe = svg::Document::new((0.0, 0.0, 200.0, 200.0));
    let mut fill = wireframe.clone();

    let mut paths = vec![];

    for c in contours {
        let p = simplify(&c);
        paths.push(p.clone());

        wireframe = wireframe.push(
            svg::Element::polyline(p)
                .fill("none")
                .set("stroke", "black")
                .set("stroke-width", "0.05"),
        );
    }

    fill = fill.push(svg::Element::path(paths).set("stroke-width", "0.05"));

    for (f, d) in &[("squares-wf.svg", wireframe), ("squares-fill.svg", fill)] {
        let mut f = File::create(f).unwrap();
        write!(f, "{}", d).unwrap();
    }
}
