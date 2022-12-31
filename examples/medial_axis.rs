// http://fourier.eng.hmc.edu/e161/lectures/morphology/node3.html

use std::{env, io::Write, path::Path};

use image::{GrayImage, Luma};

use marching_squares::simplify::simplify;
use marching_squares::svg;
use marching_squares::{march, Field};

struct Skeleton {
    pub skeleton: GrayImage,
}

impl Skeleton {
    // pixels > threshold are inside the skeleton, <= outside
    fn new(img: GrayImage, threshold: u8) -> Self {
        let (mut dt, maxd) = Self::distance_transform(img, threshold);

        let skeleton_pts = Self::find_skeleton(&dt);

        if maxd > 0 {
            for p in dt.pixels_mut() {
                p[0] = (f64::from(p[0]) / f64::from(maxd) * 255.0) as u8;
            }
        }

        let mut skeleton = GrayImage::new(dt.width(), dt.height());
        for &(x, y) in &skeleton_pts {
            skeleton.put_pixel(x, y, Luma([255]));
        }

        Self { skeleton }
    }

    fn distance_transform(mut img: GrayImage, threshold: u8) -> (GrayImage, u8) {
        for p in img.pixels_mut() {
            p[0] = u8::from(p.0[0] > threshold);
        }

        let mut maxd = 0;
        for k in 1..255 {
            let mut changed = false;

            for y in 1..img.height().saturating_sub(1) {
                for x in 1..img.width().saturating_sub(1) {
                    if img.get_pixel(x, y).0[0] != k {
                        continue;
                    }

                    let kk = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                        .iter()
                        .map(|&(x, y)| img.get_pixel(x, y).0[0])
                        .min()
                        .unwrap();

                    img.get_pixel_mut(x, y).0[0] = kk + 1;
                    changed = true;
                }
            }

            if !changed {
                maxd = k - 1;
                break;
            }
        }

        (img, maxd)
    }

    fn find_skeleton(dt: &GrayImage) -> Vec<(u32, u32)> {
        let mut points = vec![];

        for y in 1..dt.height().saturating_sub(1) {
            for x in 1..dt.width().saturating_sub(1) {
                let k = dt.get_pixel(x, y).0[0];

                if k == 0 {
                    continue;
                }

                let kk = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .map(|&(x, y)| dt.get_pixel(x, y).0[0])
                    .max()
                    .unwrap();

                if k == kk {
                    points.push((x, y));
                }
            }
        }

        points
    }
}

impl Field for Skeleton {
    fn dimensions(&self) -> (usize, usize) {
        let (w, h) = self.skeleton.dimensions();
        (w as usize, h as usize)
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        f64::from(self.skeleton.get_pixel(x as u32, y as u32).0[0])
    }
}

fn main() {
    let path = &env::args()
        .nth(1)
        .expect("please provide an input image to find the medial axis of");
    let path = Path::new(path);
    let threshold: u8 = env::args().nth(2).map_or(127, |t| t.parse().unwrap());

    let img = image::open(path)
        .expect("cannot load input image")
        .to_luma8();
    let skeleton = Skeleton::new(img, threshold);

    // skeleton.dt.save("dt.png").unwrap();

    let contours = march(&skeleton, 0.5).into_iter().map(|c| simplify(&c));

    let (w, h) = skeleton.dimensions();
    let doc = svg::Document::new((0.0, 0.0, w as f64, h as f64)).push(svg::Element::path(contours));

    let mut out = std::fs::File::create(Path::new(path.file_stem().unwrap()).with_extension("svg"))
        .expect("cannot create output file");

    write!(out, "{}", doc).expect("cannot save output");
}
