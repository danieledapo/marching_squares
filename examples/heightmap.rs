// https://tangrams.github.io/heightmapper/ is a great tool to generate heightmaps!

use std::env;
use std::io::Write;
use std::path::Path;

use marching_squares::simplify::simplify;
use marching_squares::svg;
use marching_squares::{march, Field};

#[derive(Debug)]
struct HeightMap {
    img: image::GrayImage,
}

impl HeightMap {
    fn open(p: impl AsRef<Path>) -> image::ImageResult<Self> {
        let img = image::open(p)?.to_luma();
        Ok(HeightMap { img })
    }
}

impl Field for HeightMap {
    fn dimensions(&self) -> (usize, usize) {
        let (w, h) = self.img.dimensions();
        (w as usize, h as usize)
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        f64::from(self.img.get_pixel(x as u32, y as u32).0[0])
    }
}

fn main() {
    let path = &env::args()
        .nth(1)
        .expect("please provide an input grayscale height map and optionally the number of levels");
    let path = Path::new(path);

    let nlevels = env::args()
        .nth(2)
        .map(|s| s.parse::<u8>().expect("nlevels not a number"))
        .unwrap_or(50);

    let heightmap = HeightMap::open(path).expect("cannot load height map");

    let (w, h) = heightmap.dimensions();
    let mut doc = svg::Document::new((0.0, 0.0, w as f64, h as f64));

    for i in 0..nlevels {
        let t = f64::from(i) / f64::from(nlevels - 1);
        let z = t * 255.0;
        let contours = march(&heightmap.framed(z), z)
            .into_iter()
            .map(|c| simplify(&c));

        // doc = contours.fold(doc, |d, c| {
        //     d.push(
        //         svg::Element::polyline(c)
        //             .fill("none")
        //             .set("stroke", "black")
        //             .set("stroke-width", "0.5"),
        //     )
        // });

        doc = doc.push(
            svg::Element::path(contours)
                .fill("none")
                .set("stroke", "black")
                .set("stroke-width", "0.5"),
        );
    }

    let mut out = std::fs::File::create(Path::new(path.file_stem().unwrap()).with_extension("svg"))
        .expect("cannot create output file");

    write!(out, "{}", doc).expect("cannot save output");
}
