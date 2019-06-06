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
    let countours = march(&Example {}, 0.5);

    let doc = countours
        .into_iter()
        .fold(svg::Document::new((0.0, 0.0, 200.0, 200.0)), |doc, c| {
            doc.push(
                svg::Element::polyline(c)
                    .fill("none")
                    .set("stroke", "black")
                    .set("stroke-width", "0.05"),
            )
        });

    println!("{}", doc);
}
