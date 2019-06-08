/// Simplify a given polyline by reducing the amount of points that do not actually contribute a
/// lot of details to the overall shape.
pub fn simplify(poly: &[(f64, f64)]) -> Vec<(f64, f64)> {
    simplify_with_eps(poly, 1e-9)
}

pub fn simplify_with_eps(poly: &[(f64, f64)], eps: f64) -> Vec<(f64, f64)> {
    // Ramer Douglas Peucker doesn't work with closed paths, thus simplify the open path and then
    // close it manually
    if !poly.is_empty() && poly[0] == poly[poly.len() - 1] {
        let mut s = rdp(&poly[..poly.len() - 1], eps);

        s.push(poly[poly.len() - 1]);
        s
    } else {
        rdp(poly, eps)
    }
}

/// Implementation of the [Ramer–Douglas–Peucker algorithm] to simplify an open path.
///
/// [0]: https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
fn rdp(poly: &[(f64, f64)], eps: f64) -> Vec<(f64, f64)> {
    if poly.len() < 3 {
        return poly.to_vec();
    }

    let sp = poly[0];
    let ep = *poly.last().unwrap();

    let mut farthest_i = 0;
    let mut max_dist = std::f64::NEG_INFINITY;
    for (i, p) in poly.iter().enumerate().take(poly.len() - 1).skip(1) {
        let d = perpendicular_dist(*p, (sp, ep));
        if d > max_dist {
            max_dist = d;
            farthest_i = i;
        }
    }

    if max_dist > eps {
        let mut simplified = rdp(&poly[..=farthest_i], eps);

        // remove point with max dist, it will be added with the right vec
        simplified.pop();

        simplified.extend_from_slice(&simplify_with_eps(&poly[farthest_i..], eps));

        simplified
    } else {
        vec![sp, ep]
    }
}

fn perpendicular_dist(p: (f64, f64), (s, e): ((f64, f64), (f64, f64))) -> f64 {
    let num = ((e.1 - s.1) * p.0 - (e.0 - s.0) * p.1 + e.0 * s.1 - e.1 * s.0).abs();
    let den = ((e.0 - s.0).powi(2) + (e.1 - s.0).powi(2)).sqrt();

    num / den
}
